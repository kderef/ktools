//! This file contains the `Homescreen` tool, which contains a summary of information which is important.

use std::net::{IpAddr, UdpSocket};

use crate::{
    Message,
    tool::sys_info::{FetchTask, SystemValue},
    ui::{selectable, selectable_maybe},
};

use super::*;
use iced::{
    Alignment, Background, Border, Length,
    widget::{self, Row, container, rule, space, text},
};

#[derive(Default)]
pub struct Homescreen {
    os_version: Option<Result<String, String>>,
    username: Option<Result<String, String>>,
    hostname: Option<Result<String, String>>,
    external_ip: Option<Result<String, String>>,
    primary_ip: Option<Result<String, String>>,
}

impl Tool for Homescreen {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn icon(&self) -> Text<'_> {
        icon_font::home()
    }

    fn load_data(&mut self) -> Task<Message> {
        // we don't load most of the data ourselves, rather wait for other tools to load,
        // then we catch them in update()
        Task::perform(
            async { tokio::task::spawn_blocking(get_primary_ipv4).await.unwrap() },
            Message::PrimaryIPv4Loaded,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PrimaryIPv4Loaded(result) => {
                self.primary_ip = Some(result.map(|ip| ip.to_string()));
            }
            Message::ExternalIpFetched(result) => {
                let result = result.and_then(|map| {
                    println!("{map:#?}");
                    let ip_keys = &["query", "ip"];

                    for key in ip_keys {
                        if let Some(serde_json::Value::String(s)) = map.get(*key) {
                            return Ok(s.clone());
                        }
                    }

                    Err("No IP was found".into())
                });

                self.external_ip = Some(result);
            }

            Message::SystemInfo(sys_info::Message::Fetched(task, result)) => {
                let display_result = result.map(|r| match r {
                    SystemValue::System {
                        description_long,
                        description_short: _,
                        kernel_version,
                        arch,
                        ..
                    } => {
                        let kernel_version = kernel_version
                            .strip_prefix("Windows OS Build ")
                            .unwrap_or(&kernel_version);

                        format!("{description_long} ({kernel_version}) {arch}")
                    }
                    _ => r.to_string(),
                });
                match task {
                    FetchTask::System => self.os_version = Some(display_result),
                    FetchTask::Username => self.username = Some(display_result),
                    FetchTask::Hostname => self.hostname = Some(display_result),
                    _ => {}
                }
            }
            _ => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        const FONT_SIZE: u32 = 16;

        fn info_row<'a>(
            label: &'a str,
            value: &'a Option<Result<String, String>>,
        ) -> Row<'a, Message> {
            let value_text = match value {
                None => "",
                Some(Err(e)) => e.as_str(),
                Some(Ok(v)) => v.as_str(),
            };

            widget::row![
                text(label)
                    .size(FONT_SIZE)
                    .font(BOLD_DEFAULT)
                    .width(Length::FillPortion(2)),
                space().width(Length::FillPortion(1)),
                selectable_maybe(value_text, "loading...")
                    .size(FONT_SIZE)
                    .style(if let Some(Err(_)) = value {
                        selectable::danger
                    } else {
                        selectable::style
                    })
                    .width(Length::FillPortion(5))
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill)
        }

        let separator = || rule::horizontal(1);

        let rows = widget::column![
            info_row("OS", &self.os_version),
            separator(),
            info_row("Hostname", &self.hostname),
            separator(),
            info_row("Username", &self.username),
            separator(),
            info_row("Local IP", &self.primary_ip),
            separator(),
            info_row("External IP", &self.external_ip),
        ]
        .spacing(4)
        .width(Length::Fill);

        // the card: padded, tinted background, rounded border
        let card = container(rows)
            .padding(12)
            .width(400)
            .style(|theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.weak.color)),
                    border: Border {
                        radius: 12.0.into(),
                        width: 1.0,
                        color: palette.background.strong.color,
                    },
                    ..container::Style::default()
                }
            });

        // center the card on the full available space
        container(card).center(Length::Fill).into()
    }
}

fn get_primary_ipv4() -> Result<IpAddr, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.connect("8.8.8.8:53").map_err(|e| e.to_string())?;

    let local_addr = socket.local_addr().map_err(|e| e.to_string())?;

    Ok(local_addr.ip())
}
