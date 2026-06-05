//! base utilities used by all tools

use iced::border::Radius;
use iced::font::Weight;

pub use iced_fonts::CODICON_FONT_BYTES as ICON_FONT_BYTES;
pub use iced_fonts::codicon as icon_font;

use iced::{Background, Border, Element, Font, Length, widget::*};
use iced::{Color, widget::Button};

use crate::Message;

#[allow(unused)]
pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}
#[allow(unused)]
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::from_rgba(r, g, b, a)
}
#[allow(unused)]
pub const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}
#[allow(unused)]
pub const fn rgba8(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::from_rgba8(r, g, b, a)
}

pub const BACKGROUND_TRANSPARENT: Background = Background::Color(Color::TRANSPARENT);
pub const BOLD_DEFAULT: Font = Font {
    family: iced::font::Family::SansSerif,
    weight: Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};
#[allow(unused)]
pub const MONOSPACE_DEFAULT: Font = Font {
    family: iced::font::Family::Monospace,
    weight: Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

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
                radius: 8.0.into(),
            },
            ..Default::default()
        })
}

#[inline]
pub fn content_container<'a, E: Into<Element<'a, Message>>>(inside: E) -> Container<'a, Message> {
    content_container_ex(inside, true)
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

/// hyperlink to the license
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
        /// Setting of the global app theme
        #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
        pub enum $enum_name {
            #[default]
            $(
                $name
            ),+
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                self.label().to_string()
            }
        }

        impl $enum_name {
            /// Name of the theme to be displayed
            pub const fn label(self) -> &'static str {
                match self {
                    $(
                        Self::$name => stringify!($name)
                    ),+
                }
            }
            /// List of all the themes
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

pub fn simple_button<'a>(label: &'a str, icon: Text<'a>) -> Button<'a, Message> {
    use button::Status;

    button(row![
        icon.size(15).center(),
        space().width(5),
        text(label).size(15).center()
    ])
    .style(|theme: &Theme, status| {
        let pal = theme.extended_palette();
        button::Style {
            background: Some(match status {
                Status::Active => Background::Color(pal.background.weakest.color),
                Status::Hovered => Background::Color(pal.background.strong.color),
                Status::Pressed | _ => Background::Color(pal.background.strongest.color),
            }),
            text_color: pal.background.weakest.text,
            border: Border {
                color: pal.background.base.text.scale_alpha(0.5),
                width: 1.0,
                radius: Radius::new(5.0),
            },
            ..Default::default()
        }
    })
}
