use std::io;
use std::io::IsTerminal;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub fn init(log_level: LevelFilter, log_filter: Option<String>) {
    let mut directives =
        std::env::var("RUST_LOG").unwrap_or_else(|_| format!("warn,loom={}", log_level));

    if let Some(log_filter) = log_filter {
        directives = format!("{directives},{log_filter}");
    }

    let filter = EnvFilter::try_new(&directives)
        .unwrap_or_else(|e| panic!("invalid log filter '{directives}': {e}"));

    let layer: Box<dyn Layer<_> + Send + Sync> = fmt::layer()
        .with_writer(io::stderr)
        .with_ansi(io::stderr().is_terminal())
        .boxed();

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
}
