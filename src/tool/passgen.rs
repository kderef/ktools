use super::*;

pub struct PasswordGenerator {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Tool for PasswordGenerator {
    fn name(&self) -> &'static str {
        "Password Generator"
    }

    fn background(&self) -> Color {
        Color::from_rgb(0.0, 0.2, 0.7)
    }
    fn text_color(&self) -> Color {
        Color::from_rgb(0.95, 0.95, 0.95)
    }

    fn update(&mut self, message: crate::Message) {
        // TODO
    }
    fn view(&self) -> Element<'_, crate::Message> {
        todo!()
    }
}
