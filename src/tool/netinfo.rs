use super::*;

use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{self, *},
};
use iced_aw::{
    sidebar::TabLabel,
    widget::{Sidebar, SidebarWithContent},
};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub struct NetworkInfo {
    active_tab: usize,
}

impl NetworkInfo {
    pub fn new() -> Self {
        Self { active_tab: 0 }
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
        match message {
            crate::Message::TabSelected(i) => {
                self.active_tab = i;
            }
            _ => {}
        }

        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let netconf = NetworkInterface::show().unwrap(); // TODO: fix unwrap

        let mut left_side =
            SidebarWithContent::new(crate::Message::TabSelected).set_active_tab(&self.active_tab);

        for (i, interface) in netconf.into_iter().enumerate() {
            let info = widget::column![text("A"), text("B"), text("C")];

            let view = scrollable(info);

            left_side = left_side.push(i, TabLabel::Text(interface.name), view);
        }

        let content = scrollable(left_side);

        let content = container(content).width(Length::Fill).height(Length::Fill);

        content.into()
    }
}
