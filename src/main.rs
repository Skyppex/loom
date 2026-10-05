mod audio;
mod cli;
mod log;
mod ui;

use clap::Parser;
use tracing::debug;

use crate::{cli::Cli, ui::Loom};

fn main() -> iced::Result {
    let cli = Cli::parse();
    log::init(cli.log_level, cli.log_filter);

    iced::application(Loom::new, Loom::update, Loom::view)
        .title("loom")
        .subscription(Loom::subscription)
        .run()
}
