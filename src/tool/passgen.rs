use super::*;

use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Font;
use iced::Length;
use iced::Task;
use iced::border::Radius;
use iced::clipboard;
use iced::font::Weight;
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

#[derive(Debug, Clone)]
pub enum Message {
    LengthChanged(u32),
    UseNumsToggled(bool),
    UseCharsToggled(bool),
    Regenerate,
    Copy,
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
    const SPEC: &str = "!@#$%^&*()-_=+[]{}|;:,.?";

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

        self.password.clear();
        let mut rng = rand::rng();

        for _ in 0..len_nums {
            self.password.push(
                (Self::NUMS
                    .chars()
                    .nth(rng.random_range(0..Self::NUMS.len())))
                .unwrap(),
            );
        }
        for _ in 0..len_spec {
            self.password.push(
                Self::SPEC
                    .chars()
                    .nth(rng.random_range(0..Self::SPEC.len()))
                    .unwrap(),
            );
        }
        for _ in 0..len_lower {
            self.password.push(
                Self::LOWER
                    .chars()
                    .nth(rng.random_range(0..Self::LOWER.len()))
                    .unwrap(),
            );
        }
        for _ in 0..len_upper {
            self.password.push(
                Self::UPPER
                    .chars()
                    .nth(rng.random_range(0..Self::UPPER.len()))
                    .unwrap(),
            );
        }

        let mut new = self.password.chars().collect::<Vec<char>>();

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

        let password_output = TextInput::new("password output...", &self.password)
            // .width(Length::FillPortion(3))
            .style(|theme: &Theme, _status| text_input::Style {
                background: Background::Color(Color::from_rgb8(100, 100, 100)),
                border: Border {
                    color: Color::from_rgba8(255, 255, 255, 0.5),
                    width: 1.0,
                    radius: Radius::new(5.0),
                },
                icon: Color::from_rgb8(245, 245, 245),
                placeholder: Color::from_rgba8(255, 255, 255, 0.5),
                value: theme.palette().text,
                selection: theme.palette().primary,
            })
            .size(30);

        let top_content = widget::column![
            password_output,
            widget::row![
                button(
                    container(
                        widget::row![icon_font::copy().size(24), text("copy").size(24)]
                            .spacing(10)
                            .align_y(iced::Alignment::Center),
                    )
                    .center(Length::Fill)
                )
                .on_press(crate::Message::PasswordGenerator(Message::Copy))
                .width(Length::FillPortion(3))
                .height(Length::Shrink),
                button(
                    container(
                        widget::row![icon_font::refresh().size(24), text("regenerate").size(24)]
                            .spacing(10)
                            .align_y(iced::Alignment::Center),
                    )
                    .center(Length::Fill)
                )
                .on_press(crate::Message::PasswordGenerator(Message::Regenerate))
                .width(Length::FillPortion(3))
                .height(Length::Shrink),
            ]
            .spacing(30)
        ]
        .spacing(15)
        .width(Length::FillPortion(4));

        let password_row = row![
            space().width(Length::FillPortion(1)),
            top_content,
            space().width(Length::FillPortion(1))
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
                text("Back").size(15),
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
        )
        .width(Length::Shrink)
        .on_press(crate::Message::GoHome);

        let title = text(self.name())
            .align_x(Alignment::Center)
            .size(40)
            .width(Length::Fill)
            .wrapping(text::Wrapping::None)
            .font(Font {
                weight: Weight::Bold,
                ..Default::default()
            });

        fn wrap<'a>(el: Element<'a, crate::Message>) -> Element<'a, crate::Message> {
            widget::row![
                space().width(Length::FillPortion(1)),
                container(el).width(Length::FillPortion(4)),
                space().width(Length::FillPortion(1)),
            ]
            .into()
        }

        widget::column![
            widget::row![
                go_back.width(Length::Shrink),
                space().width(Length::FillPortion(1)),
                title.width(Length::FillPortion(3)),
                space().width(Length::FillPortion(2)),
            ],
            password_row,
            wrap(length_row.into()),
            wrap(checkboxes.into()),
        ]
        .spacing(16)
        .padding(20)
        .into()
    }
}
