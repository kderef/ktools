#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::widget::*;

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(r, g, b)
}

fn main() {
    iced::application(App::new, App::update, App::view)
        .title("KTools")
        .resizable(true)
        .window_size((900, 600))
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
pub enum Message {
    Test,
}

#[derive(Debug, Clone)]
pub enum App {
    Home,
}

impl App {
    const fn new() -> Self {
        Self::Home
    }

    pub fn update(&mut self, _message: Message) {
        //
    }

    pub fn view(&self) -> Element<'_, Message> {
        let btn = |t, bg: Color, text_color: Color| {
            button(
                //
                container(text(t).size(16).color(text_color)).center(Length::Fill),
            )
            .width(Length::Fixed(160.0))
            .height(Length::Fixed(56.0))
            .on_press(Message::Test)
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
        };

        let grid = grid![
            btn(
                "Password Generator",
                rgb(0.0, 0.2, 0.95),
                rgb(0.95, 0.95, 0.95)
            ),
            btn("CMD", rgb(0.08, 0.08, 0.08), rgb(0.9, 0.9, 0.9)),
        ]
        .fluid(200)
        .spacing(20);

        let content = Container::new(grid).padding(20);
        let view = Scrollable::new(content);
        view.into()
    }
}
