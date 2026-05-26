//! base utilities used by all tools

use iced::alignment::Vertical;
use iced::border::Radius;
use iced::font::Weight;

use iced::widget;
pub use iced_fonts::CODICON_FONT_BYTES as ICON_FONT_BYTES;
pub use iced_fonts::codicon as icon_font;

use iced::{Alignment, Background, Border, Element, Font, Length, widget::*};
use iced::{Color, widget::Button};

use crate::Message;
use crate::tool::Tool;
use crate::tool::settings::Settings;

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}
pub const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}
pub const fn rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::from_rgba8(r, g, b, a)
}

pub fn go_back_button<'a>(text_size: u32) -> Button<'a, Message> {
    button(
        row![
            icon_font::arrow_left().size(text_size + 3),
            text("Back").size(text_size)
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(Message::GoHome)
}

pub fn copy_icon_btn(value: String) -> Element<'static, Message> {
    button(icon_font::copy().size(13))
        .on_press(Message::CopyToClipboard(value))
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

pub fn content_container<'a, E: Into<Element<'a, Message>>>(inside: E) -> Container<'a, Message> {
    container(scrollable(inside))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(rgb8(40, 40, 40))),
            border: Border {
                color: rgba8(255, 255, 255, 0.08),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
}

pub fn title_text<'a>(t: &'a impl Tool) -> Text<'a> {
    text(t.name()).size(28).font(Font {
        weight: Weight::Bold,
        ..Default::default()
    })
}

pub fn settings_button<'a>(settings: &'a Settings) -> Button<'a, Message> {
    button(
        container(
            row![
                space().width(Length::Fill),
                settings.icon().size(18),
                space().width(6),
                text(settings.name()).size(18),
                space().width(Length::Fill)
            ]
            .align_y(Alignment::Center),
        )
        .center(Length::Fill),
    )
    .on_press(Message::GoToSettings)
    .style(|_theme, status| widget::button::Style {
        border: Border {
            color: rgb8(160, 160, 160),
            width: 1.0,
            radius: Radius::new(6),
        },
        background: {
            let mut color = settings.background();
            match status {
                button::Status::Hovered => {
                    color.a = 0.8;
                }
                _ => {}
            }

            Some(Background::Color(color))
        },
        text_color: settings.text_color(),
        ..Default::default()
    })
}
