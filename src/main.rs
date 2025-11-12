use clap::Parser;
use st::{parser::topdown::parse_eval};
use tracing::instrument;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    input_file: Option<String>,
    #[clap(short, long)]
    expression: Option<String>,
    #[clap(long, default_value = "boot.yaml")]
    boot_file: String,
    #[clap(
        long,
        default_value = "st=warn,st::parser=warn,st::compiler=warn,st::vm=warn"
    )]
    trace_filter: String,
}

fn main() {
    let args = Args::parse();

    // Create an environment filter that can be controlled via RUST_LOG
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default filter if RUST_LOG is not set
        args.trace_filter.clone().into()
    });

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        ))
        .try_init();

    match run_main(args) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

fn run_main(args: Args) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let boot_src = std::fs::read_to_string(args.boot_file).unwrap();
    let boot = serde_yaml::from_str::<serde_yaml::Value>(&boot_src).unwrap();
    let debugger = Box::leak(Box::new(st::debug::terminal::TerminalDebugger::new()));
    let vm = Box::leak(Box::new(st::vm::VirtualMachine::new()));
    vm.set_debugger(debugger);
    if let Some(boot_map) = boot.as_mapping() {
        for (package, value) in boot_map {
            if let Some(files) = value.as_sequence() {
                for file in files {
                    let file_str = file.as_str().unwrap();
                    vm.compile_file(package.as_str().unwrap(), file_str)?;
                }
            }
        }
    } else {
        panic!("Boot file is not a valid YAML mapping");
    }

    let src = if let Some(file) = args.input_file {
        std::fs::read_to_string(file).expect("Failed to read input file")
    } else if let Some(expr) = args.expression {
        expr
    } else {
        println!("Please provide an input file or an expression to evaluate.");
        return Ok(());
    };
    let cm = st::compiler::compile_statements(&parse_eval(&src).unwrap())?;
    execute(vm, cm)
}

#[instrument(skip(vm, cm))]
fn execute(
    vm: &'static mut st::vm::VirtualMachine,
    cm: st::vm::block::CompiledMethod,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = vm.execute_method(cm)?;
    println!("{}", result);
    Ok(())
}
