use super::*;

use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::*,
};
use iced_aw::{DropDown, widget::Sidebar};
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
        let netconf = NetworkInterface::show().unwrap(); // TODO: fix unwrap

        let mut content = row![];

        for interface in netconf {
            let overlay = text(interface.name);

            let underlay =
                Column::from_iter(interface.addr.iter().map(|i| text(format!("{i:?}")).into()));

            let el = DropDown::new(overlay, underlay, false);
            content = content.push(el);
        }

        let content = container(content)
            .padding(10)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let content = scrollable(content);

        content.into()
    }
}
