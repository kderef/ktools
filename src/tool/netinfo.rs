use super::*;

use iced::widget::*;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub struct NetworkInfo {}

impl NetworkInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tool for NetworkInfo {
    fn name(&self) -> &str {
        "Network Information"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::globe()
    }
    fn background(&self) -> Color {
        Color::from_rgb8(104, 157, 106)
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }
    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let netconf = NetworkInterface::show();

        text(format!("{netconf:#?}")).into()
    }
}
