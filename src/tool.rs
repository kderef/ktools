pub use crate::base::*;

use iced::Task;

pub use iced::{Color, Element, widget::Text};
pub use iced_fonts::codicon as icon_font;

/// NOTE: a `Tool` implementation must also have `Default` to be used with `register_tools!` macro.
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

/// Automate registering modules and aggregating into `all()` function.
macro_rules! register_tools {
    ($($mod_name:ident :: $type:ident),* $(,)?) => {
        $(pub mod $mod_name;)*

        pub fn all() -> Vec<Box<dyn Tool>> {
            vec![
                $(Box::new($mod_name::$type::default()),)*
            ]
        }
    };
}

register_tools! {
    cmd::CMD,
    ext_ip::ExternalIP,
    netinfo::NetworkInfo,
    passgen::PasswordGenerator,
}
