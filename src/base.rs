//! base utilities used by all tools

pub use iced_fonts::codicon as icon_font;

use iced::{Alignment, widget::*};
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
