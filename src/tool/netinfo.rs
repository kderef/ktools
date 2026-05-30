//! Tool used for gathering **local** network information
//! For *external* network information see `src/tool/ext_ip.rs`

use crate::Message;

use super::*;
use iced::{
    Alignment, Background, Font, Length,
    widget::{self, *},
};
use iced_aw::sidebar::{Sidebar, TabLabel};
use ipconfig::{Adapter, OperStatus};
use std::net::IpAddr;

#[derive(Default)]
pub struct NetworkInfo {
    active_tab: usize,
    local_interfaces: Vec<Adapter>,
    error: Option<String>,
}

fn info_header<'a>(label: &'a str) -> Element<'a, Message> {
    text(label)
        .size(13)
        .style(text::primary)
        .font(BOLD_DEFAULT)
        .into()
}

fn info_row_ex<'a>(
    label: &'a str,
    value: impl ToString,
    bold: bool,
    label_style: fn(&Theme) -> text::Style,
) -> Element<'a, Message> {
    let value = value.to_string();
    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .style(label_style)
            .font_maybe(bold.then_some(BOLD_DEFAULT)),
        text(value.clone()).size(15).width(Length::Fill).font(Font {
            family: iced::font::Family::Monospace,
            ..Default::default()
        }),
        copy_icon_btn(value),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0])
    .into()
}

#[inline]
fn info_row<'a>(label: &'a str, value: impl ToString) -> Element<'a, Message> {
    info_row_ex(label, value, true, text::secondary)
}

#[inline]
fn info_row_primary<'a>(label: &'a str, value: impl ToString) -> Element<'a, Message> {
    info_row_ex(label, value, true, text::primary)
}

fn format_mac(mac: &[u8]) -> String {
    mac.iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(":")
}

fn iface_content<'a>(iface: &'a Adapter) -> Element<'a, Message> {
    let (status_label, status_style): (&str, fn(&Theme) -> text::Style) = match iface.oper_status()
    {
        OperStatus::IfOperStatusUp => ("up", text::success),
        OperStatus::IfOperStatusDown => ("down", text::danger),
        OperStatus::IfOperStatusTesting => ("testing", text::warning),
        OperStatus::IfOperStatusDormant => ("dormant", text::secondary),
        _ => ("unknown", text::secondary),
    };

    let top_row = row![
        text(iface.friendly_name()).size(22).font(BOLD_DEFAULT),
        space().width(Length::Fill),
        text(status_label)
            .size(20)
            .font(BOLD_DEFAULT)
            .style(status_style),
    ];

    let mut rows: Vec<Element<'a, Message>> = vec![
        top_row.into(),
        widget::column![
            info_row_primary("Type", iface.if_type().description()),
            info_row_primary("Description", iface.description()),
            info_row_primary("Name", iface.adapter_name()),
        ]
        .into(),
        rule::horizontal(1).into(),
    ];

    // MAC address
    if let Some(mac) = iface.physical_address() {
        if !mac.is_empty() {
            rows.push(info_row_primary("MAC Address", format_mac(mac)));
        }
    }
    // IP addresses
    let mut has_v4 = false;
    let mut has_v6 = false;

    for addr in iface.ip_addresses() {
        match addr {
            IpAddr::V4(v4) => {
                if !has_v4 {
                    rows.push(info_header("IPv4"));
                    has_v4 = true;
                }
                rows.push(info_row("Address", v4));

                // get prefix length for this address
                for (prefix_addr, prefix_len) in iface.prefixes() {
                    if *prefix_addr == IpAddr::V4(*v4) {
                        rows.push(info_row("Prefix Length", prefix_len));
                    }
                }
            }
            IpAddr::V6(v6) => {
                if !has_v6 {
                    rows.push(info_header("IPv6"));
                    has_v6 = true;
                }
                rows.push(info_row("Address", v6));
            }
        }
    }

    // DNS servers
    let dns_servers = iface.dns_servers();
    if !dns_servers.is_empty() {
        rows.push(info_header("DNS"));
        rows.push(info_row("Servers", dns_servers.first().unwrap()));

        for dns in &dns_servers[1..] {
            rows.push(info_row("", dns));
        }
    }

    // Gateway
    let gateways = iface.gateways();
    if !gateways.is_empty() {
        rows.push(info_row_primary("Gateways", gateways.first().unwrap()));

        for gateway in &gateways[1..] {
            rows.push(info_row("", gateway));
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
                Ok(ifs) => {
                    // sort: up interfaces first, then by friendly name
                    let mut ifs = ifs;
                    ifs.sort_by(|a, b| {
                        let a_up = a.oper_status().is_up();
                        let b_up = b.oper_status().is_up();
                        b_up.cmp(&a_up)
                            .then(a.friendly_name().cmp(b.friendly_name()))
                    });
                    self.local_interfaces = ifs;
                }
            },
            Message::Refresh => {
                return self.on_activate();
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        use iced_aw::style::Status as AwStatus;

        let mut sidebar = Sidebar::new(Message::TabSelected)
            .width(Length::Fixed(160.0))
            .height(Length::Fill)
            .style(|theme: &Theme, status| iced_aw::style::sidebar::Style {
                background: None,
                border_color: Some(rgb8(60, 60, 60)),
                border_width: 2.0,
                tab_label_background: Background::Color(match status {
                    AwStatus::Active | AwStatus::Hovered => theme.palette().primary,
                    // iced_aw::style::Status::Hovered => theme.palette().primary,
                    _ => Color::TRANSPARENT,
                }),
                tab_label_border_color: rgb8(60, 60, 60),
                tab_label_border_width: 1.0,
                icon_color: rgb8(220, 220, 220),
                icon_background: None,
                icon_border_radius: 8.0.into(),
                text_color: match status {
                    AwStatus::Active | AwStatus::Hovered => {
                        theme.extended_palette().primary.base.text
                    }
                    _ => theme.palette().text,
                },
            });

        for (i, iface) in self.local_interfaces.iter().enumerate() {
            sidebar = sidebar.push(i, TabLabel::Text(iface.friendly_name().to_string()));
        }

        sidebar = sidebar.set_active_tab(&self.active_tab);

        let content = if let Some(ref error) = self.error {
            text(format!("ERROR: {error}")).style(text::danger).into()
        } else if self.local_interfaces.is_empty() {
            text("Loading...").style(text::secondary).into()
        } else if let Some(iface) = self.local_interfaces.get(self.active_tab) {
            iface_content(iface)
        } else {
            text("No interface selected").into()
        };

        let go_back = go_back_button(13);
        let title = title_text(self);

        widget::column![
            row![go_back, space().width(16), title].align_y(Alignment::Center),
            row![sidebar, space().width(12), content].height(Length::Fill),
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}
