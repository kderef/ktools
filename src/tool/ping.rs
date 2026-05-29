use crate::Message;
use iced::{
    Alignment, Length,
    widget::{self, space, text, text_editor, text_input},
};

use super::*;

#[derive(Default)]
pub struct Ping {
    address: String,
    output: text_editor::Content,
}

impl Tool for Ping {
    fn name(&self) -> &str {
        "Ping"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::debug_disconnect()
    }
    fn background(&self, _theme: &Theme) -> Color {
        rgb8(100, 100, 100)
    }
    fn save(&self) -> Option<serde_json::Value> {
        None
    }
    fn load(&mut self, _data: serde_json::Value) {}
    fn on_activate(&mut self) -> Task<Message> {
        Task::none()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PingAddressChanged(new) => {
                self.address = new;
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let output = text_editor(&self.output)
            // .height(Length::Fill)
            .placeholder("ping output...");

        let content = widget::column![
            text_input("Address to ping...", &self.address).on_input(Message::PingAddressChanged),
            content_container(output).width(Length::Fill)
        ]
        .align_x(Alignment::Center);

        let container = content_container(content)
            .padding(12)
            .align_x(Alignment::Center);

        let col = widget::column![
            // top row
            widget::row![
                go_back_button(13),
                space().width(16),
                title_text(self).align_y(Alignment::Center) //
            ],
            space().height(10),
            container
        ]
        .padding(20);

        col.into()
    }
}
