//! Module for window handling (resizing, title bar etc.)

use iced::{
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
    border::Radius,
    mouse::Interaction,
    widget::{
        self, Button, Container, Text, button, container, mouse_area, row, space, stack, text,
    },
};

use crate::{base::*, ui::SidebarItem};
pub use iced::window::Direction;

use crate::base::{BACKGROUND_TRANSPARENT, BOLD_DEFAULT};

pub const DECORATIONS_HEIGHT: f32 = 40.0;

pub fn icon() -> Option<iced::window::Icon> {
    #[cfg(not(debug_assertions))]
    return Some({
        static RAW_DATA: &[u8] = include_bytes!("../icon_raw_rgba");

        let width = env!("ICON_RGBA_WIDTH").parse().unwrap();
        let height = env!("ICON_RGBA_HEIGHT").parse().unwrap();

        iced::window::icon::from_rgba(RAW_DATA.to_vec(), width, height).unwrap()
    });

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

pub struct WindowHandler {
    window_id: Option<iced::window::Id>,
    window_size: iced::Size,
    pub window_border_radius: f32,
    cursor_position: iced::Point,
}

impl WindowHandler {
    pub fn new() -> Self {
        Self {
            window_id: None,
            window_size: iced::Size::default(),
            window_border_radius: 0.,
            cursor_position: iced::Point::default(),
        }
    }
    pub fn handle(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::Opened { id, size } => {
                self.window_id = Some(id);
                self.window_size = size;
                return iced::window::raw_id::<crate::Message>(id)
                    .map(Message::GotRawID)
                    .map(crate::Message::Window);
            }
            Message::GotRawID(raw_id) => {
                let use_rounded = self.set_rounded_corners(raw_id);

                self.window_border_radius = if use_rounded { 8.0 } else { 0.0 };
            }
            Message::Close => {
                if let Some(id) = self.window_id {
                    return iced::window::close(id);
                }
            }
            Message::Minimize => {
                if let Some(id) = self.window_id {
                    return iced::window::minimize(id, true);
                }
            }
            Message::Drag => {
                if let Some(id) = self.window_id {
                    return iced::window::drag(id);
                }
            }
            Message::ResizeDrag(direction) => {
                if let Some(id) = self.window_id {
                    return iced::window::drag_resize(id, direction);
                }
            }
            Message::CursorMoved(position) => {
                self.cursor_position = position;
            }
        }
        Task::none()
    }

    pub fn set_rounded_corners(&self, window_id: u64) -> bool {
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
    /// Creates the margins around the windows where the user can drag to resize
    pub fn resize_areas(&self) -> [Element<'_, crate::Message>; 3] {
        let resize_area = |dir, int| {
            let f = Length::Fill;
            let m = Length::from(self.window_border_radius / 1.5);

            let (w, h) = match dir {
                Direction::North | Direction::South => (f, m),
                Direction::West | Direction::East => (m, f),
                _ => (m, m),
            };

            mouse_area(container(space()).width(w).height(h))
                .on_press(Message::ResizeDrag(dir).into())
                .interaction(int)
        };
        let n = resize_area(Direction::North, Interaction::ResizingVertically);
        let s = resize_area(Direction::South, Interaction::ResizingVertically);
        let w = resize_area(Direction::West, Interaction::ResizingHorizontally);
        let e = resize_area(Direction::East, Interaction::ResizingHorizontally);

        let nw = resize_area(Direction::NorthWest, Interaction::ResizingDiagonallyDown);
        let ne = resize_area(Direction::NorthEast, Interaction::ResizingDiagonallyUp);
        let sw = resize_area(Direction::SouthWest, Interaction::ResizingDiagonallyUp);
        let se = resize_area(Direction::SouthEast, Interaction::ResizingDiagonallyDown);

        [
            widget::column![n, space().height(Length::Fill), s].into(),
            widget::row![w, space().width(Length::Fill), e].into(),
            // corners on top of everything
            widget::column![
                widget::row![nw, space().width(Length::Fill), ne],
                space().height(Length::Fill),
                widget::row![sw, space().width(Length::Fill), se],
            ]
            .into(),
        ]
    }

    pub fn container<'a, M: 'a>(&'a self, content: impl Into<Element<'a, M>>) -> Container<'a, M> {
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                text_color: None,
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(self.window_border_radius),
                },
                ..Default::default()
            })
    }

    /// Wrap the `inside` in mouse resizing areas
    pub fn wrap<'a>(
        &'a self,
        inside: impl Into<Element<'a, crate::Message>>,
    ) -> Element<'a, crate::Message> {
        stack![mouse_area(inside).on_press(crate::Message::Window(Message::Drag)),]
            .width(Length::Fill)
            .height(Length::Fill)
            .extend(self.resize_areas())
            .into()
    }
}

pub fn decoration_button<'a, M: Into<crate::Message>, E: Into<Element<'a, crate::Message>>>(
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

pub fn titlebar_text<'a>(app: &'a crate::App) -> Text<'a> {
    let title_text = match app.selected {
        SidebarItem::Settings => "Settings",
        SidebarItem::Tool(0) => "KTools",
        SidebarItem::Tool(index) => app.tools[index].name(),
    };

    let title = text(title_text).size(30).font(BOLD_DEFAULT).center();

    title
}

pub fn decorations<'a>(
    _app: &'a crate::App,
    show_top_left_btn: bool,
) -> Element<'a, crate::Message> {
    let top_left_button = decoration_button(
        row![
            icon_font::arrow_left().size(15),
            space().width(2),
            text("back").size(15)
        ]
        .align_y(Alignment::Center)
        .height(Length::Fill),
        crate::Message::GoHome,
    );

    let top_left: Element<'_, crate::Message> = if show_top_left_btn {
        top_left_button.into()
    } else {
        space().into()
    };

    let decorations = stack![
        // title.width(Length::Fill).center(),
        row![
            top_left,
            space().width(Length::Fill),
            decoration_button(icon_font::dash().size(25), Message::Minimize),
            decoration_button(icon_font::close().size(25), Message::Close)
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
