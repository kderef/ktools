#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

#[cfg(not(debug_assertions))]
static ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

mod base;
mod tool;

use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Subscription;
use iced::Task;
use iced::clipboard;
use iced::keyboard;
use iced::widget::*;

use iced_fonts::CODICON_FONT_BYTES;

use crate::tool::Tool;

fn main() {
    iced::application(App::new, App::update, App::view)
        .window(iced::window::Settings {
            min_size: Some(iced::Size {
                width: 500.0,
                height: 400.0,
            }),
            // Avoid loading icon for faster debug build runtime
            #[cfg(not(debug_assertions))]
            icon: Some(
                iced::window::icon::from_file_data(ICON_BYTES, Some(::image::ImageFormat::Ico))
                    .unwrap(),
            ),
            ..Default::default()
        })
        .title("KTools")
        .resizable(true)
        .window_size((900, 600))
        .centered()
        .font(CODICON_FONT_BYTES)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
pub enum Message {
    /* Home page messages */
    /// Go to index
    ChooseTool(usize),
    GoHome,

    /* Generic messages */
    Refresh,
    CategorySelected(usize),
    TabSelected(usize),
    CopyToClipboard(String),
    TopTabSelected(usize),

    /* messages for passgen */
    PasswordGenerator(tool::passgen::Message),

    /* messages for ext_ip */
    ExternalIpFetched(Result<json::object::Object, String>),

    /* messages for sys_info */
    SystemInfoFetched(&'static str, Result<String, String>),
}

pub struct App {
    tools: Vec<Box<dyn Tool>>,
    selected_tool: Option<usize>,
}

fn home_button<'a>(
    icon: Text<'a>,
    name: &'a str,
    bg: Color,
    text_color: Color,
    index: usize,
) -> Button<'a, Message> {
    let icon = icon.size(28).color(text_color);
    button(
        container(
            iced::widget::column![icon, text(name).size(16).color(text_color),]
                .align_x(iced::Alignment::Center)
                .spacing(8),
        )
        .center(Length::Fill),
    )
    .width(Length::Fixed(160.0))
    .height(Length::Fixed(80.0))
    .on_press(Message::ChooseTool(index))
    .style(move |_theme: &Theme, status| {
        let alpha = match status {
            button::Status::Hovered => 0.82,
            button::Status::Pressed => 0.65,
            _ => 1.0,
        };
        let tinted = Color { a: alpha, ..bg };
        button::Style {
            snap: false,
            background: Some(Background::Color(tinted)),
            text_color,
            border: Border {
                color: Color::from_rgba(1., 1., 1., 0.3),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 6.0,
            },
        }
    })
}

impl App {
    fn new() -> Self {
        Self {
            tools: tool::all(),
            selected_tool: None,
        }
    }

    fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed {
                modified_key: keyboard::Key::Named(modified_key),
                repeat: false,
                ..
            } = event
            else {
                return None;
            };

            match modified_key {
                keyboard::key::Named::Escape => Some(Message::GoHome),
                _ => None,
            }
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
        println!("=> MESSAGE: {message:#?}");

        match message {
            Message::GoHome => {
                self.selected_tool = None;
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];

                if !tool.no_view() {
                    self.selected_tool = Some(index);
                }
                return tool.on_activate();
            }
            Message::CopyToClipboard(text) => {
                return clipboard::write(text);
            }
            other => {
                if let Some(index) = self.selected_tool {
                    return self.tools[index].update(other);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match self.selected_tool {
            Some(index) => self.tools[index].view(),
            None => {
                let children = self.tools.iter().enumerate().map(|(i, t)| {
                    home_button(t.icon(), t.name(), t.background(), t.text_color(), i).into()
                });

                let grid = Grid::with_children(children).fluid(200).spacing(20);

                let content = Container::new(grid).padding(20);
                let view = Scrollable::new(content);
                view.into()
            }
        }
    }
}
