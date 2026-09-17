use iced::widget::{column, container, Space};
use iced::{Background, Color, Element, Length};

/// A simple vertical volume bar.
/// `percent` is expected in 0.0..=1.0 and controls filled height.
pub fn view<'a, Message>(percent: f32) -> Element<'a, Message>
where
    Message: 'a,
{
    let p = percent.clamp(0.0, 1.0);
    let filled_height = p * 100.0;
    let empty_height = (1.0 - p) * 100.0;

    let filled = container(column![])
        .width(Length::Fixed(10.0))
        .height(Length::Fixed(filled_height))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.4, 0.73, 0.51))),
            ..Default::default()
        });

    let bar = column![Space::new().height(Length::Fixed(empty_height)), filled,]
        .width(Length::Fixed(10.0))
        .height(Length::Fixed(100.0));

    container(bar)
        .width(Length::Fixed(12.0))
        .height(Length::Fixed(100.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.235, 0.22, 0.21))),
            ..Default::default()
        })
        .into()
}
