//! Module for window handling (resizing, title bar etc.)

use iced::{
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
    border::Radius,
    widget::{Button, button, container, mouse_area, row, space, stack, text},
};

use crate::base::*;

pub use iced::window::Direction;

use crate::{
    Selection,
    base::{BACKGROUND_TRANSPARENT, BOLD_DEFAULT},
};

pub const DECORATIONS_HEIGHT: f32 = 40.0;

#[cfg(not(debug_assertions))]
static ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

pub fn icon() -> Option<iced::window::Icon> {
    #[cfg(not(debug_assertions))]
    return iced::window::icon::from_file_data(ICON_BYTES, Some(::image::ImageFormat::Ico))
        .unwrap();

    #[cfg(debug_assertions)]
    None
}

#[derive(Debug, Clone)]
pub enum Message {
    Opened {
        id: iced::window::Id,
        size: iced::Size,
    },
    GotRawID(u64),
    Close,
    Minimize,
    Drag,
    ResizeDrag(iced::window::Direction),
    CursorMoved(iced::Point),
}

impl Into<crate::Message> for Message {
    fn into(self) -> crate::Message {
        crate::Message::Window(self)
    }
}

pub fn handle(app: &mut crate::App, message: Message) -> Task<crate::Message> {
    match message {
        Message::Opened { id, size } => {
            app.window_id = Some(id);
            app.window_size = size;
            return iced::window::raw_id::<crate::Message>(id)
                .map(Message::GotRawID)
                .map(crate::Message::Window);
        }
        Message::GotRawID(raw_id) => {
            let use_rounded = set_rounded_corners(raw_id);

            app.window_border_radius = if use_rounded { 8 } else { 0 };
        }
        Message::Close => {
            if let Some(id) = app.window_id {
                return iced::window::close(id);
            }
        }
        Message::Minimize => {
            if let Some(id) = app.window_id {
                return iced::window::minimize(id, true);
            }
        }
        Message::Drag => {
            if let Some(id) = app.window_id {
                return iced::window::drag(id);
            }
        }
        Message::ResizeDrag(direction) => {
            if let Some(id) = app.window_id {
                return iced::window::drag_resize(id, direction);
            }
        }
        Message::CursorMoved(position) => {
            app.cursor_position = position;
        }
    }
    Task::none()
}

pub fn set_rounded_corners(window_id: u64) -> bool {
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

fn top_button<'a, M: Into<crate::Message>, E: Into<Element<'a, crate::Message>>>(
    inside: E,
    message: M,
) -> Button<'a, crate::Message> {
    use button::Status;

    let message = message.into();

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

pub fn decorations<'a>(app: &'a crate::App) -> Element<'a, crate::Message> {
    let title_text = match app.selected {
        Selection::Settings => "Settings",
        Selection::Home => "KTools",
        Selection::Tool(index) => app.tools[index].name(),
    };

    let title = text(title_text).size(30).font(BOLD_DEFAULT);

    let top_left_button = if matches!(app.selected, Selection::Home) {
        top_button(
            row![
                icon_font::settings_gear().size(15),
                space().width(4),
                text("settings").size(15).center()
            ]
            .align_y(Alignment::Center)
            .height(Length::Fill),
            crate::Message::GoToSettings,
        )
    } else {
        top_button(
            row![
                icon_font::arrow_left().size(15),
                space().width(2),
                text("back").size(15)
            ]
            .align_y(Alignment::Center)
            .height(Length::Fill),
            crate::Message::GoHome,
        )
    };

    let decorations = stack![
        title.width(Length::Fill).center(),
        row![
            top_left_button,
            space().width(Length::Fill),
            top_button(icon_font::dash().size(25), Message::Minimize),
            top_button(icon_font::close().size(25), Message::Close)
        ]
    ];

    let bar = container(decorations)
        .height(DECORATIONS_HEIGHT)
        .style(|theme: &Theme| container::Style {
            text_color: None,
            background: Some(Background::Color(
                theme.extended_palette().background.base.color,
            )),
            ..Default::default()
        });

    // enable user to move the window
    mouse_area(bar).on_press(Message::Drag.into()).into()
}
