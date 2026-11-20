use std::env;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub mod compiler;
pub mod debug;
pub mod interpreter;
pub mod memory;
pub mod parser;
pub mod vm;

pub use interpreter::{Interpreter, InterpreterError};

/// Find the project root directory (where Cargo.toml is located).
pub fn try_get_project_root() -> std::io::Result<PathBuf> {
    // Try CARGO_MANIFEST_DIR first (available during build)
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(manifest_dir));
    }

    // Fallback: walk up from current directory looking for Cargo.toml
    let mut current_dir = env::current_dir()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Ok(current_dir);
        }

        if !current_dir.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find project root (Cargo.toml not found)",
            ));
        }
    }
}

/// Get the project root directory (where Cargo.toml is located).
pub fn get_project_root() -> PathBuf {
    try_get_project_root().expect("Failed to find project root")
}

/// Get path to a file relative to project root
pub fn get_project_file_path(relative_path: &str) -> PathBuf {
    get_project_root().join(relative_path)
}

/// Get path to a test data file
pub fn get_smalltalk_file_path(area: &str, filename: &str) -> PathBuf {
    get_project_root()
        .join("smalltalk")
        .join(area)
        .join(filename)
}

/// Measure the execution time of a closure and return both the result and duration
///
/// # Example
/// ```
/// use st::time_execution;
///
/// let (result, duration) = time_execution(|| {
///     // Some expensive computation
///     2 + 2
/// });
///
/// println!("Result: {}, took: {:?}", result, duration);
/// ```
pub fn time_execution<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Measure and print the execution time of a closure
///
/// # Example
/// ```
/// use st::time_and_print;
///
/// let result = time_and_print("Computing factorial", || {
///     (1..=10).product::<u64>()
/// });
/// assert_eq!(result, 3_628_800);
/// ```
pub fn time_and_print<F, R>(description: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let (result, duration) = time_execution(f);
    println!("{}: {:?}", description, duration);
    result
}

/// Measure execution time and log it using tracing
///
/// # Example
/// ```
/// use st::time_and_trace;
///
/// let result = time_and_trace("expensive_operation", || {
///     2 + 2
/// });
/// assert_eq!(result, 4);
/// ```
pub fn time_and_trace<F, R>(operation_name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let (result, duration) = time_execution(f);
    tracing::info!("{} completed in {:?}", operation_name, duration);
    result
}

pub fn init_tracing() {
    use std::fs::OpenOptions;
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

    // Create an environment filter that can be controlled via RUST_LOG
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default filter if RUST_LOG is not set
        "trace,st::memory=warn,st::parser=warn,st::compiler=trace,st::vm=trace".into()
    });

    // Create or open the trace file
    let trace_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("st.trace")
        .expect("Failed to create st.trace file");

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(trace_file)
                .with_ansi(false) // Disable ANSI color codes for clean file output
                .with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::ENTER
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                ),
        )
        .try_init();
}
