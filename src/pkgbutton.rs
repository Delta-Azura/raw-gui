// RAW-GUI is a simple GUI interface for raw written in Rust using ICED.
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::Message;
use iced::widget::{button, text, Button};
use iced::{Background, Border, Color, Length};

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