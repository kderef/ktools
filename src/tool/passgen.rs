use super::*;

use iced::Background;
use iced::Border;
use iced::Length;
use iced::Task;
use iced::clipboard;
use iced::widget;
use iced::widget::*;

use rand::RngExt;
use rand::seq::SliceRandom;

use std::ops::RangeInclusive;

pub struct PasswordGenerator {
    length: u32,
    password: String,
    use_chars: bool,
    use_nums: bool,
}

impl PasswordGenerator {
    const LENGTH_RANGE: RangeInclusive<u32> = 8..=31;
    const SHUFFLE_TWICE: bool = false;

    const NUMS_WEIGHT: f32 = 0.25;
    const SPEC_WEIGHT: f32 = 0.15;
    const UPPER_WEIGHT: f32 = 0.25;

    // pools
    const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMS: &str = "0123456789";
    const SPEC: &str = "*+-=:()[]&";

    fn generate(&mut self) {
        // generate password
        let len_nums = if self.use_nums {
            (self.length as f32 * Self::NUMS_WEIGHT).ceil() as u32
        } else {
            0
        };
        let len_spec = if self.use_chars {
            (self.length as f32 * Self::SPEC_WEIGHT).ceil() as u32
        } else {
            0
        };
        let len_upper = (self.length as f32 * Self::UPPER_WEIGHT).ceil() as u32;
        let len_lower = self.length - len_nums - len_spec - len_upper;

        let mut generated = String::with_capacity(self.length as usize);
        let mut rng = rand::rng();

        for _ in 0..len_nums {
            generated.push(
                (Self::NUMS
                    .chars()
                    .nth(rng.random_range(0..Self::NUMS.len())))
                .unwrap(),
            );
        }
        for _ in 0..len_spec {
            generated.push(
                Self::SPEC
                    .chars()
                    .nth(rng.random_range(0..Self::SPEC.len()))
                    .unwrap(),
            );
        }
        for _ in 0..len_lower {
            generated.push(
                Self::LOWER
                    .chars()
                    .nth(rng.random_range(0..Self::LOWER.len()))
                    .unwrap(),
            );
        }
        for _ in 0..len_upper {
            generated.push(
                Self::UPPER
                    .chars()
                    .nth(rng.random_range(0..Self::UPPER.len()))
                    .unwrap(),
            );
        }

        let mut new = generated.clone().chars().collect::<Vec<char>>();

        new.shuffle(&mut rng);
        if Self::SHUFFLE_TWICE {
            new.shuffle(&mut rng);
        }

        self.password = new.iter().collect();
    }

    pub fn new() -> Self {
        let mut new = Self {
            length: 12,
            password: String::with_capacity(32),
            use_chars: true,
            use_nums: true,
        };

        new.generate();

        new
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LengthChanged(u32),
    UseNumsToggled(bool),
    UseCharsToggled(bool),
    Regenerate,
    Copy,
}

impl Tool for PasswordGenerator {
    fn name(&self) -> &'static str {
        "Password Generator"
    }

    fn icon(&self) -> Text<'_> {
        icon_font::lock()
    }

    fn background(&self) -> Color {
        rgb(0.0, 0.2, 0.7)
    }

    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        if let crate::Message::PasswordGenerator(message) = message {
            match message {
                Message::LengthChanged(new_len) => {
                    self.length = new_len;
                    self.generate();
                }
                Message::UseNumsToggled(v) => {
                    self.use_nums = v;
                    self.generate();
                }
                Message::UseCharsToggled(v) => {
                    self.use_chars = v;
                    self.generate();
                }
                Message::Regenerate => {
                    self.generate();
                }
                Message::Copy => {
                    return clipboard::write(self.password.clone());
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        let text_size = 25;

        let length_slider: Slider<'_, u32, crate::Message, Theme> =
            slider(Self::LENGTH_RANGE, self.length, |n| {
                crate::Message::PasswordGenerator(Message::LengthChanged(n))
            });

        let password_row = row![
            button(icon_font::refresh().size(text_size))
                .on_press(crate::Message::PasswordGenerator(Message::Regenerate))
                .style(|_theme: &Theme, _status| button::Style {
                    snap: false,
                    background: Some(Background::Color(rgb(1.0, 1.0, 1.0))),
                    text_color: rgb(0.06, 0.06, 0.06),
                    border: Border {
                        radius: 8.0.into(),
                        color: Color::TRANSPARENT,
                        width: 0.0,
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector { x: 0.0, y: 2.0 },
                        blur_radius: 2.0,
                    },
                }),
            TextInput::new("password output...", &self.password)
                .width(Length::FillPortion(3))
                .size(30),
            button(text("copy").size(text_size))
                .on_press(crate::Message::PasswordGenerator(Message::Copy))
                .style(|_theme: &Theme, status| button::Style {
                    snap: false,
                    background: Some(Background::Color(match status {
                        button::Status::Pressed => rgb(0.165, 0.31, 0.631),
                        _ => rgb(0.224, 0.424, 0.847),
                    })),
                    text_color: rgb(1.0, 1.0, 1.0),
                    border: Border {
                        radius: 8.0.into(),
                        color: Color::TRANSPARENT,
                        width: 0.0,
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector { x: 0.0, y: 2.0 },
                        blur_radius: 2.0,
                    },
                }),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let length_row = row![
            text(format!("Length: {}", self.length)).size(text_size),
            length_slider,
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center);

        let checkboxes = widget::column![
            checkbox(self.use_nums)
                .label("Numbers")
                .on_toggle(|v| crate::Message::PasswordGenerator(Message::UseNumsToggled(v))),
            checkbox(self.use_chars)
                .label("Special Characters")
                .on_toggle(|v| crate::Message::PasswordGenerator(Message::UseCharsToggled(v))),
        ]
        .spacing(10);

        let go_back = button(
            row![
                icon_font::arrow_left().size(text_size),
                text("Back").size(text_size),
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
        )
        .on_press(crate::Message::GoHome)
        .style(|_theme: &Theme, status| button::Style {
            snap: false,
            background: Some(Background::Color(match status {
                button::Status::Pressed => rgb(0.85, 0.85, 0.85),
                _ => rgb(0.93, 0.93, 0.93),
            })),
            text_color: rgb(0.06, 0.06, 0.06),
            border: Border {
                radius: 8.0.into(),
                color: Color::TRANSPARENT,
                width: 0.0,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 2.0,
            },
        });

        let title = text(self.name()).size(text_size);

        widget::column![
            widget::row![go_back, title],
            password_row,
            length_row,
            checkboxes,
        ]
        .spacing(16)
        .padding(20)
        .into()
    }
}
