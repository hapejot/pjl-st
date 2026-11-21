use std::{
    collections::HashMap,
    error::Error,
    hint::black_box,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

use clap::Parser;
use st::{
    Interpreter, compiler::compile_statements, memory::Value, parser::topdown::parse_eval,
    vm::VirtualMachine,
};

type BenchResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Parser)]
struct Options {
    /// Number of timed samples.
    #[arg(long, default_value_t = 5)]
    samples: usize,
    /// Executions per sample.
    #[arg(long, default_value_t = 3)]
    iterations: usize,
    /// Executions before each measurement.
    #[arg(long, default_value_t = 1)]
    warmup: usize,
    /// Also compare with Python and Node.js, which must be on PATH.
    #[arg(long)]
    compare: bool,
    /// Also measure equivalent native Rust workloads.
    #[arg(long)]
    native: bool,
    #[arg(long, hide = true)]
    bench: bool,
    #[arg(long, hide = true)]
    test: bool,
    #[arg(long, hide = true)]
    startup_child: bool,
}

struct Workload {
    name: &'static str,
    source: &'static str,
    expected: i64,
    native: fn() -> i64,
}

const WORKLOADS: &[Workload] = &[
    Workload {
        name: "integer loop (1000)",
        source: "| i sum | i := 0. sum := 0. [i < 1000] whileTrue: [i := i + 1. sum := sum + i]. ^sum",
        expected: 500_500,
        native: native_integer_loop,
    },
    Workload {
        name: "block calls (1000)",
        source: "| i block | i := 0. block := [:value | value + 1]. [i < 1000] whileTrue: [i := block value: i]. ^i",
        expected: 1_000,
        native: native_block_calls,
    },
    Workload {
        name: "method sends (1000)",
        source: "| i counter | i := 0. counter := BenchmarkCounter basicNew. [i < 1000] whileTrue: [i := counter increment: i]. ^i",
        expected: 1_000,
        native: native_method_sends,
    },
    Workload {
        name: "dictionary (1000 reads)",
        source: "| d sum | d := Dictionary new. sum := 0. 1 to: 100 do: [:i | d at: i put: i]. 1 to: 10 do: [:n | 1 to: 100 do: [:i | sum := sum + (d at: i)]]. ^sum",
        expected: 50_500,
        native: native_dictionary,
    },
    Workload {
        name: "array collect/inject (100)",
        source: "| a | a := Array new: 100. 1 to: 100 do: [:i | a at: i put: i]. ^(a collect: [:each | each * 2]) inject: 0 into: [:sum :each | sum + each]",
        expected: 10_100,
        native: native_array_collect_inject,
    },
];

const COUNTER_CLASS: &str = "Object subclass: BenchmarkCounter [ increment: value [ ^value + 1 ] ]";

fn native_integer_loop() -> i64 {
    let mut i = 0;
    let mut sum = 0;
    while i < 1_000 {
        i = black_box(i) + 1;
        sum += i;
    }
    sum
}

fn native_block_calls() -> i64 {
    let mut i = 0;
    let block = |value| value + 1;
    while i < 1_000 {
        i = block(black_box(i));
    }
    i
}

struct NativeCounter;

impl NativeCounter {
    fn increment(&self, value: i64) -> i64 {
        value + 1
    }
}

fn native_method_sends() -> i64 {
    let mut i = 0;
    let counter = NativeCounter;
    while i < 1_000 {
        i = counter.increment(black_box(i));
    }
    i
}

fn native_dictionary() -> i64 {
    let mut values = HashMap::new();
    let mut sum = 0;
    for i in 1..=100_i64 {
        values.insert(black_box(i), i);
    }
    for _ in 0..10 {
        for i in 1..=100_i64 {
            sum += values[&black_box(i)];
        }
    }
    sum
}

fn native_array_collect_inject() -> i64 {
    let mut values = vec![0; 100];
    for i in 1..=100 {
        values[i - 1] = black_box(i as i64);
    }
    let collected: Vec<_> = values.into_iter().map(|each| each * 2).collect();
    black_box(collected).into_iter().sum()
}

fn bootstrap_vm(root: &Path) -> BenchResult<VirtualMachine> {
    let vm = VirtualMachine::try_new()?;
    let manifest: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(root.join("boot.yaml"))?)?;
    let packages = manifest.as_mapping().ok_or("Boot file must be a mapping")?;
    for (package, files) in packages {
        let package = package.as_str().ok_or("Boot package must be a string")?;
        let files = files
            .as_sequence()
            .ok_or("Boot package must contain a list")?;
        for file in files {
            let file = file.as_str().ok_or("Boot file entry must be a string")?;
            let path = root.join("smalltalk").join(package).join(file);
            let source = std::fs::read_to_string(&path)
                .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;
            vm.compile_src(&source)
                .map_err(|error| format!("Failed to load {}: {error}", path.display()))?;
        }
    }
    Ok(vm)
}

fn measure<T>(
    options: &Options,
    mut operation: impl FnMut() -> BenchResult<T>,
    mut validate: impl FnMut(T) -> BenchResult<()>,
) -> BenchResult<Duration> {
    for _ in 0..options.warmup {
        validate(black_box(operation()?))?;
    }
    let mut samples = Vec::with_capacity(options.samples);
    for _ in 0..options.samples {
        let mut elapsed = Duration::ZERO;
        for _ in 0..options.iterations {
            let start = Instant::now();
            let value = operation()?;
            elapsed += start.elapsed();
            validate(black_box(value))?;
        }
        samples.push(elapsed.as_secs_f64() / options.iterations as f64);
    }
    samples.sort_by(f64::total_cmp);
    let midpoint = samples.len() / 2;
    let median = if samples.len() % 2 == 0 {
        (samples[midpoint - 1] + samples[midpoint]) / 2.0
    } else {
        samples[midpoint]
    };
    let median = Duration::from_secs_f64(median);
    if median.is_zero() {
        return Err("Measured duration is zero; increase the workload or timer resolution".into());
    }
    Ok(median)
}

fn integer_result(value: Value, expected: i64, name: &str) -> BenchResult<()> {
    match value {
        Value::Integer(actual) if actual == expected => Ok(()),
        other => Err(format!("{name}: expected {expected}, got {other:?}").into()),
    }
}

fn report(name: &str, duration: Duration) {
    let seconds = duration.as_secs_f64();
    println!(
        "{name:<38} {:>12.6} ms  {:>12.1} runs/s",
        seconds * 1_000.0,
        1.0 / seconds
    );
}

fn checked_output(command: &mut Command) -> BenchResult<std::process::Output> {
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!(
            "{command:?} failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(output)
}

fn compare(options: &Options, root: &Path) -> BenchResult<()> {
    let executable = std::env::current_exe()?;
    println!("\nProcess startup + trivial program (includes process exit and captured output)");
    for (name, program, args) in [
        (
            "  Smalltalk",
            executable.as_os_str(),
            vec!["--startup-child"],
        ),
        (
            "  Python",
            std::ffi::OsStr::new("python"),
            vec!["-c", "print(1)"],
        ),
        (
            "  Node.js",
            std::ffi::OsStr::new("node"),
            vec!["-e", "console.log(1)"],
        ),
    ] {
        report(
            name,
            measure(
                options,
                || checked_output(Command::new(program).args(&args).current_dir(root)),
                |output| {
                    if String::from_utf8(output.stdout)?.trim() != "1" {
                        return Err(format!("{name}: unexpected startup result").into());
                    }
                    Ok(())
                },
            )?,
        );
    }

    for (name, program, script) in [
        ("Python", "python", "compare.py"),
        ("Node.js", "node", "compare.js"),
    ] {
        println!("\n{name} workload execution:");
        io::stdout().flush()?;
        let status = Command::new(program)
            .arg(root.join("benches").join(script))
            .args([
                options.samples.to_string(),
                options.iterations.to_string(),
                options.warmup.to_string(),
            ])
            .current_dir(root)
            .status()?;
        if !status.success() {
            return Err(format!("{name} comparison failed ({status})").into());
        }
    }
    Ok(())
}

fn main() -> BenchResult<()> {
    let mut options = Options::parse();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if options.startup_child {
        integer_result(Interpreter::new(&root)?.eval("^1")?, 1, "startup")?;
        println!("1");
        return Ok(());
    }
    if options.test {
        options.samples = 1;
        options.iterations = 1;
        options.warmup = 0;
    }
    if options.samples == 0 || options.iterations == 0 {
        return Err("--samples and --iterations must be greater than zero".into());
    }
    println!(
        "Single-threaded; median of {} samples, {} runs/sample, {} warmup runs",
        options.samples, options.iterations, options.warmup
    );
    println!(
        "Times per complete run (workload setup included); no debugger or tracing subscriber."
    );

    report(
        "bootstrap (Interpreter::new)",
        measure(&options, || Ok(Interpreter::new(&root)?), |_| Ok(()))?,
    );

    let interpreter = Interpreter::new(&root)?;
    interpreter.load_source(COUNTER_CLASS)?;
    let vm = bootstrap_vm(&root)?;
    vm.compile_src(COUNTER_CLASS)?;

    for workload in WORKLOADS {
        let syntax = parse_eval(workload.source)?;
        let method = compile_statements(&syntax)?;
        integer_result(
            vm.execute_method(method.clone())?,
            workload.expected,
            workload.name,
        )?;
        integer_result(
            interpreter.eval(workload.source)?,
            workload.expected,
            workload.name,
        )?;

        println!("\n{}", workload.name);
        report(
            "  parse",
            measure(
                &options,
                || Ok(parse_eval(black_box(workload.source))?),
                |_| Ok(()),
            )?,
        );
        report(
            "  compile (parsed input)",
            measure(
                &options,
                || Ok(compile_statements(black_box(&syntax))?),
                |_| Ok(()),
            )?,
        );
        report(
            "  execute (compiled bytecode)",
            measure(
                &options,
                || vm.execute_method(black_box(method.clone())),
                |value| integer_result(value, workload.expected, workload.name),
            )?,
        );
        report(
            "  Interpreter::eval (end-to-end)",
            measure(
                &options,
                || Ok(interpreter.eval(black_box(workload.source))?),
                |value| integer_result(value, workload.expected, workload.name),
            )?,
        );
    }
    if options.native || options.compare {
        const BATCH_SIZE: u32 = 100;
        println!("\nNative Rust workload execution (100 runs per timer interval):");
        for workload in WORKLOADS {
            let batch_time = measure(
                &options,
                || {
                    let mut results = [0; BATCH_SIZE as usize];
                    for result in &mut results {
                        *result = black_box((workload.native)());
                    }
                    Ok(results)
                },
                |results| {
                    for actual in results {
                        if actual != workload.expected {
                            return Err(format!(
                                "{}: expected {}, got {actual}",
                                workload.name, workload.expected
                            )
                            .into());
                        }
                    }
                    Ok(())
                },
            )?;
            let duration = batch_time / BATCH_SIZE;
            if duration.is_zero() {
                return Err("Native per-run duration is below timer resolution".into());
            }
            report(&format!("  {}", workload.name), duration);
        }
    }
    if options.compare {
        compare(&options, &root)?;
    }
    Ok(())
}
