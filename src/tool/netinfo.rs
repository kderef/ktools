use super::*;
use iced::{
    Alignment, Background, Border, Font, Length,
    font::Weight,
    widget::{self, *},
};
use iced_aw::{sidebar::TabLabel, widget::SidebarWithContent};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub struct NetworkInfo {
    active_tab: usize,
    interfaces: Vec<NetworkInterface>,
}

impl NetworkInfo {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            interfaces: vec![],
        }
    }
}

fn info_row<'a>(label: &'a str, value: impl ToString) -> Element<'a, crate::Message> {
    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .color(Color::from_rgb8(160, 160, 160)),
        text(value.to_string()).size(15),
    ]
    .padding([5, 0])
    .into()
}

fn iface_content<'a>(iface: &'a NetworkInterface) -> Element<'a, crate::Message> {
    let mut rows: Vec<Element<'a, crate::Message>> = vec![
        text(&iface.name)
            .size(22)
            .font(Font {
                weight: Weight::Bold,
                ..Default::default()
            })
            .into(),
        rule::horizontal(1).into(),
    ];

    if let Some(ref mac) = iface.mac_addr {
        rows.push(info_row("MAC Address", mac.clone()));
    }

    for addr in &iface.addr {
        match addr {
            network_interface::Addr::V4(v4) => {
                rows.push(
                    text("IPv4")
                        .size(13)
                        .color(Color::from_rgb8(104, 157, 106))
                        .into(),
                );
                rows.push(info_row("Address", v4.ip));
                if let Some(m) = v4.netmask {
                    rows.push(info_row("Netmask", m));
                }
                if let Some(b) = v4.broadcast {
                    rows.push(info_row("Broadcast", b));
                }
            }
            network_interface::Addr::V6(v6) => {
                rows.push(
                    text("IPv6")
                        .size(13)
                        .color(Color::from_rgb8(104, 157, 106))
                        .into(),
                );
                rows.push(info_row("Address", v6.ip));
                if let Some(m) = v6.netmask {
                    rows.push(info_row("Netmask", m));
                }
            }
        }
    }

    container(scrollable(column(rows).spacing(4).padding([12, 16])))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(40, 40, 40))),
            border: Border {
                color: Color::from_rgba8(255, 255, 255, 0.08),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .into()
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

    fn on_select(&mut self) {
        self.interfaces = NetworkInterface::show().unwrap_or_default();
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        if let crate::Message::TabSelected(i) = message {
            self.active_tab = i;
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        let mut sidebar =
            SidebarWithContent::new(crate::Message::TabSelected).sidebar_style(|_theme, status| {
                iced_aw::style::sidebar::Style {
                    background: None,
                    border_color: None,
                    border_width: 0.0,
                    tab_label_background: Background::Color(match status {
                        iced_aw::style::Status::Active => Color::from_rgb8(60, 60, 60),
                        iced_aw::style::Status::Hovered => Color::from_rgb8(70, 70, 70),
                        _ => Color::TRANSPARENT,
                    }),
                    tab_label_border_color: Color::TRANSPARENT,
                    tab_label_border_width: 0.0,
                    icon_color: Color::from_rgb8(220, 220, 220),
                    icon_background: None,
                    icon_border_radius: 8.0.into(),
                    text_color: Color::from_rgb8(220, 220, 220),
                }
            });

        for (i, iface) in self.interfaces.iter().enumerate() {
            sidebar = sidebar.push(i, TabLabel::Text(iface.name.clone()), iface_content(iface));
        }

        sidebar = sidebar.set_active_tab(&self.active_tab);

        let go_back = button(
            row![icon_font::arrow_left().size(18), text("Back").size(15)]
                .spacing(6)
                .align_y(Alignment::Center),
        )
        .on_press(crate::Message::GoHome);

        let title = text(self.name()).size(28).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        widget::column![
            row![go_back, space().width(16), title].align_y(Alignment::Center),
            container(sidebar).width(Length::Fill).height(Length::Fill),
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}
