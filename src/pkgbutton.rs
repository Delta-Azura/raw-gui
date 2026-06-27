pub fn pkg_button(label: &str, active: bool) -> Button<'_, Message> {
    button(
        text(label).size(13)
    )
    .on_press(Message::Select(label.to_string()))
    .width(Length::Fill)
    .style(move |_theme, status| {
        let bg = if active {
            Color::from_rgb(0.11, 0.62, 0.46)
        } else if matches!(status, button::Status::Hovered) {
            Color::from_rgb(0.15, 0.15, 0.17)
        } else {
            Color::from_rgb(0.10, 0.10, 0.12)
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: if active {
                Color::WHITE
            } else {
                Color::from_rgb(0.75, 0.75, 0.78)
            },
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
}