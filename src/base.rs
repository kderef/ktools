//! base utilities used by all tools

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
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::from_rgba(r, g, b, a)
}
pub const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}
pub const fn rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::from_rgba8(r, g, b, a)
}

pub const BOLD_DEFAULT: Font = Font {
    family: iced::font::Family::SansSerif,
    weight: Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};
pub const MONOSPACE_DEFAULT: Font = Font {
    family: iced::font::Family::Monospace,
    weight: Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

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
        .style(|theme: &Theme, status| {
            let pal = theme.extended_palette();

            button::Style {
                snap: false,
                background: match status {
                    button::Status::Disabled => None,
                    button::Status::Active => Some(Background::Color(pal.background.weak.color)),
                    button::Status::Hovered => {
                        Some(Background::Color(pal.background.strongest.color))
                    }
                    button::Status::Pressed => {
                        Some(Background::Color(pal.background.weakest.color))
                    }
                },
                text_color: pal.background.weakest.text,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
            }
        })
        .into()
}

pub fn content_container_ex<'a, E: Into<Element<'a, Message>>>(
    inside: E,
    inside_scrollable: bool,
) -> Container<'a, Message> {
    let inside = if inside_scrollable {
        scrollable(inside).into()
    } else {
        inside.into()
    };

    container(inside)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Background::Color(
                theme.extended_palette().background.weakest.color,
            )),
            border: Border {
                color: match theme {
                    Theme::Light => rgba8(0, 0, 0, 0.3),
                    _ => rgba8(255, 255, 255, 0.08),
                },
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
}

#[inline]
pub fn content_container<'a, E: Into<Element<'a, Message>>>(inside: E) -> Container<'a, Message> {
    content_container_ex(inside, true)
}

pub fn title_text<'a>(t: &'a impl Tool) -> Text<'a> {
    text(t.name()).size(28).font(BOLD_DEFAULT)
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
    .style(|theme: &Theme, status| widget::button::Style {
        border: Border {
            color: rgb8(160, 160, 160),
            width: 1.0,
            radius: Radius::new(6),
        },
        background: {
            let mut color = theme.extended_palette().primary.base.color;
            match status {
                button::Status::Hovered => {
                    color.a = 0.8;
                }
                _ => {}
            }

            Some(Background::Color(color))
        },
        text_color: theme.extended_palette().primary.base.text,
        ..Default::default()
    })
}

pub fn hyperlink<'a>(label: &'static str, link: Option<&'static str>) -> Button<'a, Message> {
    use button::Status;

    let url = link.unwrap_or(label);

    button(text(label).size(15))
        .on_press(Message::OpenURL(url))
        .style(|theme: &Theme, status| {
            let pal = theme.extended_palette();
            button::Style {
                background: None,
                text_color: match status {
                    Status::Pressed => pal.primary.weak.color,
                    Status::Active => pal.primary.base.color,
                    Status::Hovered => pal.primary.strong.color,
                    Status::Disabled => pal.secondary.base.text,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(0),
                },
                ..Default::default()
            }
        })
        .padding(0)
}

/// Link to the app's source code
pub fn source_link<'a>() -> Button<'a, Message> {
    const SOURCE_LINK: &str = env!("CARGO_PKG_REPOSITORY");
    hyperlink(SOURCE_LINK, None)
}

pub fn license_link<'a>() -> Button<'a, Message> {
    const LICENSE: &str = env!("CARGO_PKG_LICENSE");
    hyperlink(
        LICENSE,
        Some("https://www.gnu.org/licenses/gpl-3.0.en.html"),
    )
}

#[inline]
pub fn app_version<'a>() -> Row<'a, Message> {
    row![
        text(env!("CARGO_PKG_VERSION")).size(15).style(text::base),
        space().width(10),
        text(format!("({})", env!("GIT_HASH")))
            .size(15)
            .style(text::secondary)
    ]
}

/// Macro for src/tool/settings.rs
/// NOTE: the first element is the Default.
#[macro_export]
macro_rules! define_themes {
    ($enum_name:ident { $($name:ident => $iced_theme:expr),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
        pub enum $enum_name {
            #[default]
            $(
                $name
            ),+
        }

        impl $enum_name {
            pub const fn label(self) -> &'static str {
                match self {
                    $(
                        Self::$name => stringify!($name)
                    ),+
                }
            }
            pub const fn all() -> &'static [Self] {
                &[
                    $(Self::$name),+
                ]
            }
        }
        impl Into<iced::Theme> for $enum_name {
            fn into(self) -> iced::Theme {
                match self {
                    $(
                        Self::$name => $iced_theme
                    ),+
                }
            }
        }
    };
}
