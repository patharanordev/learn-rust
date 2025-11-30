use tracing_subscriber::{
    self,
    filter::EnvFilter,
    fmt::layer,
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use std::env;

pub fn init_tracer() -> Result<(), String> {
    let env_filter = env::var("RUST_LOG").unwrap_or_else(|_| {
        "debug".into()
    });

    tracing_subscriber::registry()
        .with(EnvFilter::new(env_filter))
        .with(
            layer()
            .with_target(true)
            .with_line_number(true)
        )
        .init();

    Ok(())
}