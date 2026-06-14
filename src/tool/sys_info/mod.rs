mod process;
mod system_value;
mod tasks;

pub use process::ProcessOpen;
pub use system_value::{Bytes, Disk, SystemValue};
use tasks::*;

use std::fmt;
use std::{collections::HashMap, os::windows::process::CommandExt};

use iced::{
    Alignment, Background, Border, Length, Theme,
    widget::{self, button, progress_bar, row, space, text},
};
use sysinfo::System;

use super::*;
use crate::Message;

pub struct SystemInfo {
    /// `None` means loading, `Some(Result<...>)` will be received upon `Message::SystemInfoFetched`
    info: HashMap<&'static str, Option<Result<SystemValue, String>>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            info: TASKS.iter().map(|(k, _)| (*k, None)).collect(),
        }
    }
}

impl Tool for SystemInfo {
    fn name(&self) -> &'static str {
        "System Information"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::vm()
    }

    fn on_activate(&mut self) -> Task<crate::Message> {
        // Launch tasks for each of the TASKS
        Task::batch(TASKS.iter().map(|(name, fetch)| {
            Task::perform(async move { fetch() }, move |result| {
                Message::SystemInfoFetched(name, result)
            })
        }))
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SystemInfoFetched(key, result) => {
                if let Some(val) = self.info.get_mut(&key) {
                    *val = Some(result);
                }
            }
            Message::Refresh => {
                for v in self.info.values_mut() {
                    *v = None;
                }
                return self.on_activate();
            }
            Message::SystemInfoOpen(proc) => {
                let cmd = proc.command();
                let prog = cmd[0];
                let args = &cmd[1..];

                #[cfg(windows)]
                use windows::Win32::System::Threading::CREATE_NO_WINDOW;

                let _result = std::process::Command::new(prog)
                    .creation_flags(CREATE_NO_WINDOW.0)
                    .args(args)
                    .spawn();

                #[cfg(debug_assertions)]
                println!("$ {prog} {args:?} -> {_result:?}");
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let mut rows = widget::column![].spacing(2).height(Length::Fill);

        // Iterate through TASKS instead of self.info to preserve order
        for (key, _) in TASKS {
            let value = &self.info[key];
            rows = rows.push(info_row(key, value));
        }

        let proc_button = |label, msg: ProcessOpen| {
            simple_button(label, msg.icon()).on_press(Message::SystemInfoOpen(msg))
        };

        let spacing = 10;

        rows = rows
            .push(space().height(Length::Fill))
            .push(
                row![
                    proc_button("Control Panel", ProcessOpen::ConfigPanel),
                    proc_button("Printers", ProcessOpen::Printers),
                    proc_button("Admin Tools", ProcessOpen::AdminTools),
                    proc_button("Windows Features", ProcessOpen::Features),
                ]
                .spacing(spacing)
                .align_y(Alignment::End),
            )
            .push(space().height(spacing - 2))
            .push(
                row![
                    proc_button("Computer Management", ProcessOpen::ComputerManagement),
                    proc_button("Power Management", ProcessOpen::PowerOptions),
                    proc_button("Installed Apps", ProcessOpen::InstalledApps),
                ]
                .spacing(spacing)
                .align_y(Alignment::End),
            );

        let container = content_container_ex(rows, false)
            .padding(12)
            .height(Length::Fill);

        // When all info loaded, enable the buttons
        let all_loaded = self.info.values().all(Option::is_some);

        let bottom_row = row![
            button(text("refresh").size(24).center())
                .on_press_maybe(all_loaded.then_some(Message::Refresh))
                .width(Length::Fill),
            space().width(10),
            button(text("copy all").size(24).center())
                .width(Length::Fill)
                .on_press_maybe(all_loaded.then_some({
                    let text = TASKS
                        .iter()
                        .filter_map(|(k, _)| {
                            if let Some(Ok(val)) = &self.info[k] {
                                Some(format!("{k}: {}", val.to_string()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    Message::CopyToClipboard(text)
                }))
        ];
        let col = widget::column![
            // middle
            container,
            // bottom
            space().height(20),
            bottom_row
        ];
        col.height(Length::Fill).padding(12).into()
    }
}

fn info_row<'a>(
    key: &str,
    value: &'a Option<Result<SystemValue, String>>,
) -> Element<'a, crate::Message> {
    let label = text(key.to_string()).size(14).width(120);

    let content: Element<'a, crate::Message> = match value {
        None => text("Loading...").size(14).style(text::secondary).into(),
        Some(Err(e)) => text(format!("ERROR: {e}"))
            .size(14)
            .style(text::danger)
            .into(),
        Some(Ok(v)) => v.widget(),
    };

    row![label, content, space().width(20)]
        .align_y(Alignment::Center)
        .padding([6, 0])
        .into()
}
