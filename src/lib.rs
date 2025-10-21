pub mod compiler;
pub mod memory;
pub mod parser;
pub mod vm;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
