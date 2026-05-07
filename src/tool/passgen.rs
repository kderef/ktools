use std::ops::RangeInclusive;

use iced::widget;
use iced::widget::*;

use super::*;

pub struct PasswordGenerator {
    length: u32,
    password: String,
    use_chars: bool,
    use_nums: bool,
}

impl PasswordGenerator {
    const LENGTH_RANGE: RangeInclusive<u32> = 8..=31;

    pub fn new() -> Self {
        Self {
            length: 12,
            password: String::from("hello"),
            use_chars: true,
            use_nums: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LengthChanged(u32),
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

    fn update(&mut self, message: crate::Message) {
        if let crate::Message::PasswordGenerator(message) = message {
            match message {
                Message::LengthChanged(new_len) => {
                    self.length = new_len;
                    // TODO: regenerate
                }
            }
        }
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let slider = slider(Self::LENGTH_RANGE, self.length, |n| {
            crate::Message::PasswordGenerator(Message::LengthChanged(n))
        });

        widget::column![
            //
            TextInput::new("123abc", &self.password),
            slider
        ]
        .into()
    }
}
