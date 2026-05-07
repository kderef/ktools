use super::*;

pub struct CMD;

impl Tool for CMD {
    fn name(&self) -> &'static str {
        "CMD"
    }
    fn no_view(&self) -> bool {
        true
    }

    fn background(&self) -> Color {
        Color::from_rgb(0.08, 0.08, 0.08)
    }
    fn text_color(&self) -> Color {
        Color::from_rgb(0.9, 0.9, 0.9)
    }

    fn update(&mut self, message: crate::Message) {
        unreachable!()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        unreachable!()
    }
}
