#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

mod tool;

use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Task;
use iced::widget::*;
use iced_fonts::CODICON_FONT_BYTES;

use crate::tool::Tool;

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}

fn main() {
    iced::application(App::new, App::update, App::view)
        .title("KTools")
        .resizable(true)
        .window_size((900, 600))
        .font(CODICON_FONT_BYTES)
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
pub enum Message {
    /// Go to index
    ChooseTool(usize),
    GoHome,
    PasswordGenerator(tool::passgen::Message),
}

pub struct App {
    tools: [Box<dyn Tool>; 2],
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
            tools: [
                Box::new(tool::cmd::CMD),
                Box::new(tool::passgen::PasswordGenerator::new()),
            ],
            selected_tool: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
        println!("=> MESSAGE: {message:?}");

        match message {
            Message::GoHome => {
                self.selected_tool = None;
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];
                tool.on_select();

                if !tool.no_view() {
                    self.selected_tool = Some(index);
                }
            }
            other => {
                if let Some(index) = self.selected_tool {
                    return self.tools[index].update(other);
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
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
