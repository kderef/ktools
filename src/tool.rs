pub use iced::{Color, Element};

pub mod cmd;
pub mod passgen;

pub trait Tool {
    fn name(&self) -> &str;
    fn background(&self) -> Color;
    fn text_color(&self) -> Color;

    fn no_view(&self) -> bool {
        false
    }

    fn update(&mut self, message: crate::Message);
    fn view(&self) -> Element<'_, crate::Message>;
}
