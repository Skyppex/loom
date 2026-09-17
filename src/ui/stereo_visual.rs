use iced::widget::{column, container, row};
use iced::{Background, Color, Element, Length};

use super::volume_visual;

/// Stereo meter showing two `volume_visual`s side-by-side.
pub fn view<'a, Message>(left_volume: f32, right_volume: f32) -> Element<'a, Message>
where
    Message: 'a,
{
    let divider = container(column![])
        .width(Length::Fixed(1.0))
        .height(Length::Fixed(100.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.235, 0.22, 0.21))),
            ..Default::default()
        });

    row![
        volume_visual::view(left_volume),
        divider,
        volume_visual::view(right_volume),
    ]
    .spacing(2)
    .align_y(iced::Alignment::Center)
    .padding(5)
    .into()
}
