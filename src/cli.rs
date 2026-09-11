use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    pub device_name: Option<String>,
}
