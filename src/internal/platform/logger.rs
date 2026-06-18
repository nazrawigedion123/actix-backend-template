use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initializes the global tracing subscriber with structured JSON formatting.
/// This acts as the equivalent to your Go Zap production logger setup.
pub fn init_logger() {
    // 1. Define log filtering levels from the environment (RUST_LOG).
    // Fallback defaults to 'info' globally, and 'debug' for your app module.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,backend_actix_template=debug"));

    // 2. Configure JSON formatting layer (Structured logging like Uber Zap)
    let json_layer = fmt::layer()
        .json()
        .with_current_span(true) // Automatically includes request context fields
        .with_target(true)       // Includes the module paths where log occurred
        .with_thread_ids(false); // Keeps log outputs clean and microservice friendly

    // 3. Register the subscriber globally across the entire async runtime
    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .init();
}

