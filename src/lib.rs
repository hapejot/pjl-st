use std::env;
use std::path::PathBuf;

pub mod compiler;
pub mod memory;
pub mod parser;
pub mod vm;

/// Get the project root directory (where Cargo.toml is located)
pub fn get_project_root() -> PathBuf {
    // Try CARGO_MANIFEST_DIR first (available during build)
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        return PathBuf::from(manifest_dir);
    }

    // Fallback: walk up from current directory looking for Cargo.toml
    let mut current_dir = env::current_dir().expect("Failed to get current directory");
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return current_dir;
        }

        if !current_dir.pop() {
            panic!("Could not find project root (Cargo.toml not found)");
        }
    }
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

pub fn init_tracing() {
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

    // Create an environment filter that can be controlled via RUST_LOG
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default filter if RUST_LOG is not set
        "st=warn,st::parser=warn,st::compiler=trace,st::vm=trace".into()
    });

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        ))
        .try_init();
}
