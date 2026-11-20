use clap::Parser;
use std::io::IsTerminal;
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    input_file: Option<String>,
    #[clap(short, long)]
    expression: Option<String>,
    #[clap(long, default_value = "boot.yaml")]
    boot_file: String,
    /// Run with the interactive fullscreen debugger
    #[clap(long)]
    debug: bool,
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
    if args.debug && !(std::io::stdin().is_terminal() && std::io::stdout().is_terminal()) {
        return Err("--debug requires a terminal on stdin and stdout".into());
    }

    let interpreter = st::Interpreter::with_boot_file(
        &args.boot_file,
        st::try_get_project_root()?.join("smalltalk"),
    )?;
    if args.debug {
        interpreter.set_debugger(Arc::new(st::debug::terminal::TerminalDebugger::new()))?;
    }

    let src = if let Some(file) = args.input_file {
        std::fs::read_to_string(file)?
    } else if let Some(expr) = args.expression {
        expr
    } else {
        println!("Please provide an input file or an expression to evaluate.");
        return Ok(());
    };
    let result = interpreter.eval(&src)?;
    println!("{}", result);
    Ok(())
}
