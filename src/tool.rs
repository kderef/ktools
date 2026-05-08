pub use crate::rgb;

use iced::Task;

pub use iced::{Color, Element, widget::Text};
pub use iced_fonts::codicon as icon_font;

pub mod cmd;
pub mod netinfo;
pub mod passgen;

pub trait Tool {
    fn name(&self) -> &str;
    fn icon(&self) -> Text<'_>;
    fn background(&self) -> Color;
    fn text_color(&self) -> Color;

    /// Run code when the tool is selected
    fn on_select(&mut self) {}

    /// Should the tool's view() be used?
    fn no_view(&self) -> bool {
        false
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message>;
    fn view(&self) -> Element<'_, crate::Message>;
}
