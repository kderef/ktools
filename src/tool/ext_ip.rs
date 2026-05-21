use iced::widget::text;

use super::*;

#[derive(Default)]
pub struct ExternalIP {}

impl Tool for ExternalIP {
    fn name(&self) -> &str {
        "External IP"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::broadcast()
    }
    fn background(&self) -> Color {
        rgb8(100, 100, 100) // TODO
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }
    fn on_select(&mut self) {}
    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        text("Hello world!").into()
    }
}
