use clap::Parser;
use tracing::level_filters::LevelFilter;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    pub device_name: Option<String>,

    /// Minimum level for loom's own logs: trace, debug, info, warn, error, off.
    /// Imported crates stay at warn unless overridden with --log-filter
    #[arg(long, default_value = "debug")]
    pub log_level: LevelFilter,

    /// Extra log filter directives in tracing's EnvFilter syntax, applied on
    /// top of the defaults. E.g. "russh=info" or "russh=trace,tokio_tungstenite=debug"
    #[arg(long)]
    pub log_filter: Option<String>,
}
