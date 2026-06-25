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
}

impl Tool for Homescreen {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn icon(&self) -> Text<'_> {
        icon_font::home()
    }

    fn load_data(&mut self) -> Task<Message> {
        // we don't load the data ourselves, rather wait for other tools to load,
        // then we catch them in update()
        Task::none()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
                    .width(Length::FillPortion(1)),
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
            info_row("Host", &self.hostname),
            separator(),
            info_row("Username", &self.username),
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
