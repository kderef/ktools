//! base utilities used by all tools

use iced::border::Radius;
use iced::font::Weight;

pub use iced_fonts::CODICON_FONT_BYTES as ICON_FONT_BYTES;
pub use iced_fonts::codicon as icon_font;

use iced::{Alignment, Background, Border, Element, Font, Length, widget::*};
use iced::{Color, widget::Button};

use crate::App;
use crate::Message;
use crate::Selection;

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

pub const BACKGROUND_TRANSPARENT: Background = Background::Color(Color::TRANSPARENT);
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

pub fn set_window_rounded_corners(window_id: u64) -> bool {
    #[cfg(windows)]
    unsafe {
        use std::ffi::c_void;

        use windows::Win32::Foundation::HWND;
        use windows::Win32::Graphics::Dwm::*;
        use windows::Win32::UI::Controls::MARGINS;

        let hwnd = HWND(window_id as *mut _);
        let preference = DWMWCP_ROUND;

        let succeeded = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &preference as *const _ as *const c_void,
            size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        )
        .is_ok();

        let margins = MARGINS {
            cxLeftWidth: 1,
            cxRightWidth: 1,
            cyTopHeight: 0, // hides title bar
            cyBottomHeight: 1,
        };
        let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
        succeeded
    }
}

pub const WINDOW_DECORATIONS_HEIGHT: f32 = 40.0;

/// Window decoration button
fn window_button<'a, E: Into<Element<'a, Message>>>(
    inside: E,
    message: Message,
) -> Button<'a, Message> {
    use button::Status;
    button(inside)
        .on_press(message)
        .style(|theme: &Theme, status| {
            let pal = theme.extended_palette();
            button::Style {
                background: Some(BACKGROUND_TRANSPARENT),
                text_color: match status {
                    Status::Active => pal.background.weakest.text,
                    Status::Hovered => pal.background.strongest.text,
                    Status::Pressed | _ => pal.background.base.text,
                },
                border: Border {
                    width: 0.0,
                    color: Color::TRANSPARENT,
                    radius: Radius::new(0),
                },
                ..Default::default()
            }
        })
}

pub fn window_decorations<'a>(app: &'a App) -> Element<'a, Message> {
    let title_text = match app.selected {
        Selection::Settings => "Settings",
        Selection::Home => "KTools",
        Selection::Tool(index) => app.tools[index].name(),
    };

    let title = text(title_text).size(30).font(BOLD_DEFAULT);

    let top_left_button = if matches!(app.selected, Selection::Home) {
        window_button(
            row![
                icon_font::settings_gear().size(15),
                space().width(4),
                text("settings").size(15).center()
            ]
            .align_y(Alignment::Center)
            .height(Length::Fill),
            Message::GoToSettings,
        )
    } else {
        window_button(
            row![
                icon_font::arrow_left().size(15),
                space().width(2),
                text("back").size(15)
            ]
            .align_y(Alignment::Center)
            .height(Length::Fill),
            Message::GoHome,
        )
    };

    let decorations = stack![
        title.width(Length::Fill).center(),
        row![
            top_left_button,
            space().width(Length::Fill),
            window_button(icon_font::dash().size(25), Message::WindowMinimize),
            window_button(icon_font::close().size(25), Message::WindowClose)
        ]
    ];

    let bar = container(decorations)
        .height(WINDOW_DECORATIONS_HEIGHT)
        .style(|theme: &Theme| container::Style {
            text_color: None,
            background: Some(Background::Color(
                theme.extended_palette().background.base.color,
            )),
            ..Default::default()
        });

    // enable user to move the window
    mouse_area(bar).on_press(Message::WindowDrag).into()
}
