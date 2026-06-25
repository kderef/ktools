mod process;
mod system_value;
mod tasks;

pub use process::ProcessOpen;
pub use system_value::{Bytes, Disk, SystemValue};

use std::collections::BTreeMap;
use std::fmt;

use iced::{
    Alignment, Background, Border, Length, Theme,
    widget::{self, button, progress_bar, row, space, text},
};
use sysinfo::System;

use super::*;
pub use tasks::FetchTask;
pub use tasks::Message;

pub struct SystemInfo {
    /// `None` means loading, `Some(Result<...>)` will be received upon `Message::SystemInfoFetched`
    /// NOTE: BTreeMap because we care about the order
    info: BTreeMap<FetchTask, Option<Result<SystemValue, String>>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            info: FetchTask::all().iter().map(|ft| (*ft, None)).collect(),
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

    fn load_data(&mut self) -> Task<crate::Message> {
        // Launch tasks for each of the TASKS
        Task::batch(FetchTask::all().iter().map(|ft| {
            Task::perform(async move { (ft.action())() }, move |result| {
                Message::Fetched(*ft, result).into()
            })
        }))
    }

    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::none()
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        let crate::Message::SystemInfo(message) = message else {
            return Task::none();
        };

        match message {
            Message::Fetched(task, result) => {
                if let Some(val) = self.info.get_mut(&task) {
                    *val = Some(result);
                }
            }
            Message::OpenProcess(proc) => {
                let cmd = proc.command();
                let prog = cmd[0];
                let args = &cmd[1..];

                let mut process = std::process::Command::new(prog);

                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    use windows::Win32::System::Threading::CREATE_NO_WINDOW;

                    let flags = CREATE_NO_WINDOW.0;
                    process.creation_flags(flags);
                }

                let _result = process.args(args).spawn();

                #[cfg(debug_assertions)]
                println!("$ {prog} {args:?} -> {_result:?}");
            }
            Message::Refresh => {
                // Delete existing data, then start task to fetch new data
                for v in self.info.values_mut() {
                    *v = None;
                }
                return self.load_data();
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let info_rows = self.info.iter().map(|(k, v)| info_row(k.name(), v));
        let mut rows = widget::column(info_rows).spacing(2).height(Length::Fill);

        let proc_button = |label, msg: ProcessOpen| {
            simple_button(label, msg.icon()).on_press(Message::OpenProcess(msg).into())
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
                .on_press_maybe(all_loaded.then_some(Message::Refresh.into()))
                .width(Length::Fill),
            space().width(10),
            button(text("copy all").size(24).center())
                .width(Length::Fill)
                .on_press_maybe(all_loaded.then_some({
                    let text = self
                        .info
                        .iter()
                        .filter_map(|(k, v)| {
                            if let Some(Ok(val)) = v {
                                Some(format!("{k}: {}", val.to_string()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    crate::Message::CopyToClipboard(text)
                }))
        ];
        let col = widget::column![
            container,
            space().height(20), //
            bottom_row
        ];
        col.height(Length::Fill).padding(12).into()
    }
}

fn info_row<'a>(
    key: &'a str,
    value: &'a Option<Result<SystemValue, String>>,
) -> Element<'a, crate::Message> {
    let label = text(key).size(14).width(120);

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
