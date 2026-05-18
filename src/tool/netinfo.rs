use super::*;
use iced::{
    Alignment, Background, Border, Font, Length,
    font::Weight,
    widget::{self, *},
};
use iced_aw::sidebar::{Sidebar, TabLabel};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

const CATEGORY_LOCAL: usize = 0;
const CATEGORY_EXTERNAL: usize = 1;

pub struct NetworkInfo {
    active_tab: usize,
    active_category: usize,
    local_interfaces: Vec<NetworkInterface>,
    external_ipv4: Option<String>,
}

impl NetworkInfo {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            active_category: CATEGORY_LOCAL,
            local_interfaces: vec![],
            external_ipv4: None,
        }
    }
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

fn content_container<'a, E: Into<Element<'a, crate::Message>>>(
    inside: E,
) -> Container<'a, crate::Message> {
    container(scrollable(inside))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(rgb8(40, 40, 40))),
            border: Border {
                color: rgba8(255, 255, 255, 0.08),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
}

fn external_content<'a>(addr: &str) -> Element<'a, crate::Message> {
    let top = text("External Addresses").size(22).font(Font {
        weight: Weight::Bold,
        ..Default::default()
    });

    let mut rows: Vec<Element<'a, crate::Message>> = vec![top.into(), rule::horizontal(1).into()];

    content_container(column(rows).spacing(4).padding([12, 16])).into()
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

    fn on_select(&mut self) {
        self.local_interfaces = NetworkInterface::show().unwrap_or_default();
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            crate::Message::TabSelected(i) => {
                self.active_category = CATEGORY_LOCAL;
                self.active_tab = i;
            }
            crate::Message::CategorySelected(i) => self.active_category = i,
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
                tab_label_background: if self.active_category == CATEGORY_LOCAL {
                    Background::Color(match status {
                        iced_aw::style::Status::Active => rgb8(44, 94, 173),
                        iced_aw::style::Status::Hovered => rgb8(44, 94, 173), // rgb8(21, 145, 220),
                        _ => Color::TRANSPARENT,
                    })
                } else {
                    Background::Color(Color::TRANSPARENT)
                },
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

        // Sections
        let sections = Sidebar::new(crate::Message::CategorySelected)
            .width(Length::Fixed(160.0))
            .height(Length::Shrink)
            .style(|_theme, status| iced_aw::style::sidebar::Style {
                background: None,
                border_color: Some(rgb8(60, 60, 60)),
                border_width: 2.0,
                tab_label_background: Background::Color(match status {
                    iced_aw::style::Status::Active => rgb8(44, 94, 173),
                    iced_aw::style::Status::Hovered => rgb8(21, 145, 220),
                    _ => Color::TRANSPARENT,
                }),
                tab_label_border_color: rgb8(60, 60, 60),
                tab_label_border_width: 1.0,
                icon_color: rgb8(220, 220, 220),
                icon_background: None,
                icon_border_radius: 8.0.into(),
                text_color: rgb8(220, 220, 220),
            })
            .push(CATEGORY_LOCAL, TabLabel::Text("Local".to_owned()))
            .push(CATEGORY_EXTERNAL, TabLabel::Text("External".to_owned()))
            .set_active_tab(&self.active_category);

        let left_side = widget::column![sections, space().height(30), sidebar];

        let content = match self.active_category {
            CATEGORY_LOCAL => {
                if let Some(iface) = self.local_interfaces.get(self.active_tab) {
                    iface_content(iface)
                } else {
                    text("No interface selected").into()
                }
            }
            CATEGORY_EXTERNAL => external_content("hello"),
            _ => unreachable!(),
        };

        let go_back = go_back_button(13);

        let title = text(self.name()).size(28).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        widget::column![
            row![go_back, space().width(16), title].align_y(Alignment::Center),
            row![left_side, space().width(12), content,].height(Length::Fill),
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}
