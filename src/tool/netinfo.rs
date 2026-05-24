use super::*;
use iced::{
    Alignment, Background, Font, Length,
    font::Weight,
    widget::{self, *},
};
use iced_aw::sidebar::{Sidebar, TabLabel};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

#[derive(Default)]
pub struct NetworkInfo {
    active_tab: usize,
    local_interfaces: Vec<NetworkInterface>,
}

fn info_row<'a>(label: &'a str, value: impl ToString) -> Element<'a, crate::Message> {
    let value = value.to_string();
    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(value.clone()).size(15).width(Length::Fill),
        copy_icon_btn(value),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0])
    .into()
}

fn iface_content<'a>(iface: &'a NetworkInterface) -> Element<'a, crate::Message> {
    let top_row = row![
        text(&iface.name).size(22).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        }),
        space().width(Length::Fill),
        {
            let (desc, color) = match iface.internal {
                true => ("( internal )", rgb8(253, 218, 13)),
                false => ("( public )", rgb8(0, 180, 0)),
            };
            text(desc).size(20).color(color).font(Font {
                weight: Weight::Bold,
                ..Default::default()
            })
        }
    ];

    let mut rows: Vec<Element<'a, crate::Message>> =
        vec![top_row.into(), rule::horizontal(1).into()];

    if let Some(ref mac) = iface.mac_addr {
        rows.push(info_row("MAC Address", mac.clone()));
    }

    for addr in &iface.addr {
        match addr {
            network_interface::Addr::V4(v4) => {
                rows.push(text("IPv4").size(13).color(rgb8(104, 157, 106)).into());
                rows.push(info_row("Address", v4.ip));
                if let Some(m) = v4.netmask {
                    rows.push(info_row("Netmask", m));
                }
                if let Some(b) = v4.broadcast {
                    rows.push(info_row("Broadcast", b));
                }
            }
            network_interface::Addr::V6(v6) => {
                rows.push(text("IPv6").size(13).color(rgb8(104, 157, 106)).into());
                rows.push(info_row("Address", v6.ip));
                if let Some(m) = v6.netmask {
                    rows.push(info_row("Netmask", m));
                }
            }
        }
    }

    content_container(column(rows).spacing(4).padding([12, 16])).into()
}

impl Tool for NetworkInfo {
    fn name(&self) -> &str {
        "Network Information"
    }

    fn icon(&self) -> Text<'_> {
        icon_font::globe()
    }

    fn background(&self) -> Color {
        rgb8(104, 157, 106)
    }

    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }

    fn on_activate(&mut self) -> Task<crate::Message> {
        // TODO: create task
        self.local_interfaces = NetworkInterface::show().unwrap_or_default();
        Task::none()
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            crate::Message::TabSelected(i) => self.active_tab = i,
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        let mut sidebar = Sidebar::new(crate::Message::TabSelected)
            .width(Length::Fixed(160.0))
            .height(Length::Fill)
            .style(|_theme, status| iced_aw::style::sidebar::Style {
                background: None,
                border_color: Some(rgb8(60, 60, 60)),
                border_width: 2.0,
                tab_label_background: Background::Color(match status {
                    iced_aw::style::Status::Active => rgb8(44, 94, 173),
                    iced_aw::style::Status::Hovered => rgb8(44, 94, 173), // rgb8(21, 145, 220),
                    _ => Color::TRANSPARENT,
                }),
                tab_label_border_color: rgb8(60, 60, 60),
                tab_label_border_width: 1.0,
                icon_color: rgb8(220, 220, 220),
                icon_background: None,
                icon_border_radius: 8.0.into(),
                text_color: rgb8(220, 220, 220),
            });

        for (i, iface) in self.local_interfaces.iter().enumerate() {
            sidebar = sidebar.push(i, TabLabel::Text(iface.name.clone()));
        }

        sidebar = sidebar.set_active_tab(&self.active_tab);

        let content = if let Some(iface) = self.local_interfaces.get(self.active_tab) {
            iface_content(iface)
        } else {
            text("No interface selected").into()
        };

        let go_back = go_back_button(13);
        let title = title_text(self);

        widget::column![
            row![go_back, space().width(16), title].align_y(Alignment::Center),
            row![sidebar, space().width(12), content,].height(Length::Fill),
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}
