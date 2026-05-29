use super::*;

use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Font;
use iced::Length;
use iced::Task;
use iced::border::Radius;
use iced::widget;
use iced::widget::*;

use rand::RngExt;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;

use std::ops::RangeInclusive;

#[derive(Serialize, Deserialize)]
pub struct PasswordGenerator {
    length: u32,
    #[serde(skip)]
    password: String,
    use_chars: bool,
    use_nums: bool,
    special_chars: String,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        let mut new = Self {
            length: 16,
            password: String::with_capacity(32),
            use_chars: true,
            use_nums: true,
            special_chars: Self::DEFAULT_SPEC_CHARS.to_owned(),
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
    ChangeSpecialCharacters(String),
    ResetSpecialCharacers,
}

impl Into<crate::Message> for Message {
    fn into(self) -> crate::Message {
        crate::Message::PasswordGenerator(self)
    }
}

impl PasswordGenerator {
    const LENGTH_RANGE: RangeInclusive<u32> = 8..=31;
    const SHUFFLE_TWICE: bool = false;

    const NUMS_WEIGHT: f32 = 0.25;
    const UPPER_WEIGHT: f32 = 0.25;
    const SPEC_WEIGHT: f32 = 0.25;

    // pools
    const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMS: &str = "0123456789";
    const DEFAULT_SPEC_CHARS: &str = "!@#$%^&*()-_=+[]{}|;:,.?";

    fn generate(&mut self) {
        // generate password
        let len_nums = if self.use_nums {
            (self.length as f32 * Self::NUMS_WEIGHT).ceil() as u32
        } else {
            0
        };
        let len_spec = if self.use_chars && self.special_chars.len() > 0 {
            (self.length as f32 * Self::SPEC_WEIGHT).ceil() as u32
        } else {
            0
        };
        let len_upper = (self.length as f32 * Self::UPPER_WEIGHT).ceil() as u32;
        let len_lower = self.length - len_nums - len_spec - len_upper;

        self.password.clear();
        let mut rng = rand::rng();

        // Push `len` amount of random characters from `chars`
        let mut push_rand = |len, chars: &str| {
            for _ in 0..len {
                self.password
                    .push(chars.chars().nth(rng.random_range(0..chars.len())).unwrap());
            }
        };

        push_rand(len_nums, Self::NUMS);
        push_rand(len_spec, &self.special_chars);
        push_rand(len_lower, Self::LOWER);
        push_rand(len_upper, Self::UPPER);

        let mut new = self.password.chars().collect::<Vec<char>>();

        new.shuffle(&mut rng);
        if Self::SHUFFLE_TWICE {
            new.shuffle(&mut rng);
        }

        self.password = new.iter().collect();
    }
}

impl Tool for PasswordGenerator {
    fn name(&self) -> &'static str {
        "Password Generator"
    }

    fn icon(&self) -> Text<'_> {
        icon_font::lock()
    }

    fn background(&self, theme: &Theme) -> Color {
        // rgb(0.0, 0.2, 0.7)
        theme.extended_palette().primary.strong.color
    }

    fn save(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
    fn load(&mut self, data: serde_json::Value) {
        let Ok(loaded) = serde_json::from_value(data) else {
            return;
        };

        *self = loaded;
        self.generate(); // populate password field
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

                Message::ChangeSpecialCharacters(new) => {
                    // Keep only visible ASCII non-alphanumeric characters
                    let filtered: String = new
                        .chars()
                        .filter(|c| {
                            c.is_ascii() && !c.is_ascii_alphanumeric() && !c.is_ascii_whitespace()
                        })
                        .collect();

                    // Remove duplicates while preserving order
                    let mut unique = String::new();

                    for c in filtered.chars() {
                        if !unique.contains(c) {
                            unique.push(c);
                        }
                    }

                    let final_chars = unique;

                    if self.special_chars != final_chars {
                        self.special_chars = final_chars;
                        self.generate();
                    }
                }

                Message::ResetSpecialCharacers => {
                    if self.special_chars != Self::DEFAULT_SPEC_CHARS {
                        self.special_chars = Self::DEFAULT_SPEC_CHARS.to_owned();
                        self.generate();
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        let text_size = 25;

        let length_slider: Slider<'_, u32, crate::Message, Theme> =
            slider(Self::LENGTH_RANGE, self.length, |n| {
                Message::LengthChanged(n).into()
            });

        let password_output = text_input("password output...", &self.password)
            // .width(Length::FillPortion(3))
            .style(|theme: &Theme, _status| text_input::Style {
                background: Background::Color(theme.extended_palette().background.strong.color),
                border: Border {
                    color: theme.palette().text,
                    width: 1.0,
                    radius: Radius::new(5.0),
                },
                icon: rgb8(245, 245, 245),
                placeholder: rgba8(255, 255, 255, 0.5),
                value: theme.palette().text,
                selection: theme.extended_palette().primary.weak.color,
            })
            .font(Font {
                family: iced::font::Family::Serif,
                ..Default::default()
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
                .on_press_with(|| crate::Message::CopyToClipboard(self.password.clone()))
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
                .on_press(Message::Regenerate.into())
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
                .on_toggle(|v| Message::UseNumsToggled(v).into()),
            widget::row![
                checkbox(self.use_chars)
                    .label("Special Characters")
                    .on_toggle(|v| Message::UseCharsToggled(v).into()),
                text_input("example: !@#$%", &self.special_chars)
                    .size(15)
                    .on_input_maybe(
                        self.use_chars
                            .then_some(|s| Message::ChangeSpecialCharacters(s).into())
                    ),
                button("reset").on_press(Message::ResetSpecialCharacers.into())
            ]
            .spacing(15)
        ]
        .spacing(10);

        let go_back = go_back_button(text_size);

        let title = text(self.name())
            .align_x(Alignment::Center)
            .size(40)
            .width(Length::Fill)
            .wrapping(text::Wrapping::None)
            .font(BOLD_DEFAULT);

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
