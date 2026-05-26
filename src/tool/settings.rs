use iced::widget::text;

use super::*;

#[derive(Default)]
pub struct Settings {}

impl Tool for Settings {
    fn name(&self) -> &str {
        "Settings"
    }
    fn hidden(&self) -> bool {
        true
    }
    fn icon(&self) -> Text<'_> {
        icon_font::settings_gear()
    }
    fn background(&self) -> Color {
        rgb8(0, 100, 180)
    }
    fn save(&self) -> Option<serde_json::Value> {
        todo!()
    }
    fn load(&mut self, _data: serde_json::Value) {}

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        text("Hello world!").into()
    }
}
