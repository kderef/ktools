pub use crate::rgb;
pub use iced::{Color, Element};
pub use iced_fonts::codicon as icon_font;

pub mod cmd;
pub mod passgen;

pub trait Tool {
    fn name(&self) -> &str;
    fn icon(&self) -> Element<'_, crate::Message>;
    fn background(&self) -> Color;
    fn text_color(&self) -> Color;

    /// Run code when the tool is selected
    fn on_select(&mut self) {}

    /// Should the tool's view() be used?
    fn no_view(&self) -> bool {
        false
    }

    fn update(&mut self, message: crate::Message);
    fn view(&self) -> Element<'_, crate::Message>;
}
