//! Tool used for gathering **local** network information
//! For *external* network information see `src/tool/ext_ip.rs`

use std::net::{IpAddr, Ipv4Addr};

use crate::Message;

use super::*;
use iced::{
    Alignment, Background, Length,
    widget::{self, *},
};
use iced_aw::sidebar::{Sidebar, TabLabel};
use ipconfig::{Adapter, OperStatus};
// use network_interface::{NetworkInterface, NetworkInterfaceConfig};

#[derive(Default)]
pub struct NetworkInfo {
    active_tab: usize,
    local_interfaces: Vec<Adapter>,
    error: Option<String>,
}

fn info_row<'a>(label: &'a str, value: impl ToString) -> Element<'a, Message> {
    let value = value.to_string();
    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .style(text::secondary),
        // .color(rgb8(160, 160, 160)),
        text(value.clone()).size(15).width(Length::Fill),
        copy_icon_btn(value),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0])
    .into()
}

fn iface_content<'a>(iface: &'a Adapter) -> Element<'a, Message> {
    let top_row = row![
        text(iface.friendly_name()).size(22).font(BOLD_DEFAULT),
        space().width(Length::Fill),
        {
            let (desc, style): (&str, fn(&Theme) -> text::Style) = match iface.oper_status() {
                OperStatus::IfOperStatusUp | _ => ("online", text::success),
            };
            text(desc).size(20).font(BOLD_DEFAULT).style(style)
        }
    ];

    let mut rows: Vec<Element<'a, Message>> = vec![top_row.into(), rule::horizontal(1).into()];

    if let Some(mac) = iface.physical_address() {
        let mac_str = match mac {
            &[o1, o2, o3, o4] => Ipv4Addr::from_octets([o1, o2, o3, o4]).to_string(),
            _ => String::from("unknown"),
        };

        rows.push(info_row("MAC Address", mac_str));
    }

    for addr in iface.ip_addresses() {
        match addr {
            IpAddr::V4(v4) => {
                rows.push(
                    text("IPv4")
                        .size(13)
                        .style(text::primary)
                        .font(BOLD_DEFAULT)
                        .into(),
                ); // .color(rgb8(104, 157, 106)).into());
                rows.push(info_row("Address", v4.ip));
                // if let Some(m) = v4 {
                //     rows.push(info_row("Netmask", m));
                // }
                if let Some(b) = v4.broadcast {
                    rows.push(info_row("Broadcast", b));
                }
            }
            IpAddr::V6(v6) => {
                rows.push(
                    text("IPv6")
                        .size(13)
                        .style(text::primary)
                        .font(BOLD_DEFAULT)
                        .into(),
                ); //.color(rgb8(104, 157, 106)).into());

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

    fn background(&self, theme: &Theme) -> Color {
        // rgb8(104, 157, 106)
        theme.extended_palette().success.strong.color
    }

    fn on_activate(&mut self) -> Task<Message> {
        Task::perform(
            async { ipconfig::get_adapters().map_err(|e| e.to_string()) },
            Message::NetworkInterfacesFetched,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(i) => self.active_tab = i,
            Message::NetworkInterfacesFetched(result) => match result {
                Err(e) => self.error = Some(e),
                Ok(ifs) => self.local_interfaces = ifs,
            },
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mut sidebar = Sidebar::new(Message::TabSelected)
            .width(Length::Fixed(160.0))
            .height(Length::Fill)
            .style(|theme: &Theme, status| iced_aw::style::sidebar::Style {
                background: None,
                border_color: Some(rgb8(60, 60, 60)),
                border_width: 2.0,
                tab_label_background: Background::Color(match status {
                    iced_aw::style::Status::Active => theme.palette().primary,
                    iced_aw::style::Status::Hovered => theme.palette().primary, // rgb8(21, 145, 220),
                    _ => Color::TRANSPARENT,
                }),
                tab_label_border_color: rgb8(60, 60, 60),
                tab_label_border_width: 1.0,
                icon_color: rgb8(220, 220, 220),
                icon_background: None,
                icon_border_radius: 8.0.into(),
                text_color: theme.palette().text,
            });

        for (i, iface) in self.local_interfaces.iter().enumerate() {
            sidebar = sidebar.push(i, TabLabel::Text(iface.name.clone()));
        }

        sidebar = sidebar.set_active_tab(&self.active_tab);

        let content = if let Some(ref error) = self.error {
            text(format!("ERROR: {error}")).style(text::danger).into()
        } else if let Some(iface) = self.local_interfaces.get(self.active_tab) {
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
