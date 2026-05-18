//! base utilities used by all tools

pub use iced_fonts::codicon as icon_font;

use iced::{Alignment, Background, Border, Element, widget::*};
use iced::{Color, widget::Button};

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}
pub const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}
pub const fn rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::from_rgba8(r, g, b, a)
}

pub fn go_back_button<'a>(text_size: u32) -> Button<'a, crate::Message> {
    button(
        row![
            icon_font::arrow_left().size(text_size + 3),
            text("Back").size(text_size)
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(crate::Message::GoHome)
}

pub fn copy_icon_btn(value: String) -> Element<'static, crate::Message> {
    button(icon_font::copy().size(13))
        .on_press(crate::Message::CopyToClipboard(value))
        .padding([2, 6])
        .style(|_theme: &Theme, status| button::Style {
            snap: false,
            background: match status {
                button::Status::Disabled => None,
                button::Status::Active => Some(Background::Color(rgb8(60, 60, 60))),
                button::Status::Hovered => Some(Background::Color(rgb8(80, 80, 80))),
                button::Status::Pressed => Some(Background::Color(rgb8(0, 100, 10))),
            },
            text_color: rgb8(200, 200, 200),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Default::default(),
        })
        .into()
}
