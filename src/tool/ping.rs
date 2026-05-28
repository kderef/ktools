use crate::Message;
use iced::{
    Alignment,
    widget::{self, space, text},
};

use super::*;

#[derive(Default)]
pub struct Ping {}

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
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let content = widget::column![text("Hello")];

        let container = content_container(content).padding(12);

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
