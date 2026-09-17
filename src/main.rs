mod audio;
mod cli;
mod ui;

use crate::ui::Loom;

fn main() -> iced::Result {
    iced::application(Loom::new, Loom::update, Loom::view)
        .title("loom")
        .subscription(Loom::subscription)
        .run()
}
