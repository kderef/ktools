use super::*;

pub struct PasswordGenerator {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Tool for PasswordGenerator {
    fn name(&self) -> &'static str {
        "Password Generator"
    }
    fn icon(&self) -> Element<'_, crate::Message> {
        icon_font::lock()
            .size(28)
            .color(rgb(0.95, 0.95, 0.95))
            .into()
    }

    fn background(&self) -> Color {
        rgb(0.0, 0.2, 0.7)
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }

    fn update(&mut self, message: crate::Message) {
        // TODO
    }
    fn view(&self) -> Element<'_, crate::Message> {
        todo!()
    }
}
