use std::collections::HashMap;

use iced::{
    Alignment, Length,
    widget::{self, button, row, space, text},
};
use sysinfo::System;

use super::*;
use crate::Message;

#[derive(Debug, Clone)]
struct Cpu {
    name: String,
    brand: String,
    vendor_id: String,
    frequency: u64,
    usage: f32,
}
impl From<&sysinfo::Cpu> for Cpu {
    fn from(value: &sysinfo::Cpu) -> Self {
        Self {
            vendor_id: value.vendor_id().to_owned(),
            name: value.name().to_owned(),
            brand: value.brand().to_owned(),
            frequency: value.frequency(),
            usage: value.cpu_usage(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemValue {
    Text(String),
    Cpus(Vec<Cpu>),
    Memory {
        total_bytes: u64,
        free_bytes: u64,
        used_bytes: u64,
    },
}

static TASKS: &[(&str, fn() -> Result<SystemValue, String>)] = &[
    //
    ("hostname", fetch_hostname),
    ("username", fetch_username),
    ("CPU model", fetch_cpu),
    ("Total RAM", fetch_ram),
];

pub struct SystemInfo {
    info: HashMap<&'static str, Option<Result<SystemValue, String>>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            info: HashMap::from_iter(TASKS.iter().map(|(k, _)| (*k, None))),
        }
    }
}

impl Tool for SystemInfo {
    fn name(&self) -> &str {
        "System Information"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::vm()
    }
    fn background(&self) -> Color {
        rgb8(66, 123, 88)
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::batch(TASKS.iter().map(|(k, f)| {
            Task::perform(async move { f() }, move |r| {
                Message::SystemInfoFetched(k, r)
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
                for (_, v) in &mut self.info {
                    *v = None;
                }
                return self.on_activate();
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let mut rows = widget::column![];

        for (k, v) in &self.info {
            rows = rows.push(info_row(k, v));
        }

        let container = content_container(rows).padding(12).height(Length::Fill);
        let go_back = go_back_button(13);
        let title = title_text(self);

        let mut col = widget::column![
            widget::row![go_back, space().width(16), title.align_y(Alignment::Center)]
                .align_y(Alignment::Center),
            space().height(10),
            container
        ];

        if self.info.values().all(|v| v.is_some()) {
            let bottom_row = row![
                button(text("refresh").size(24).center())
                    .on_press(Message::Refresh)
                    .width(Length::Fill),
                space().width(10),
                button(text("copy all").size(24).center())
                    .width(Length::Fill)
                    .on_press_with(|| {
                        let text = self
                            .info
                            .iter()
                            .filter_map(|(k, v)| {
                                if let Some(Ok(val)) = v {
                                    Some(format!("{k}: {val:?}")) // TODO: fix
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        Message::CopyToClipboard(text)
                    })
            ];

            col = col.push(space().height(20)).push(bottom_row);
        }

        col.height(Length::Fill).padding(12).into()
    }
}

fn info_row<'a>(
    key: &str,
    value: &Option<Result<SystemValue, String>>,
) -> Element<'a, crate::Message> {
    let (value_text, value_color) = match value {
        None => ("Loading...".to_string(), rgb8(140, 140, 140)),
        Some(Ok(v)) => (v.clone(), rgb8(200, 200, 200)),
        Some(Err(e)) => (format!("ERROR: {e}"), rgb8(220, 10, 10)),
    };

    let mut row = row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(value_text.clone())
            .size(15)
            .width(Length::Fill)
            .color(value_color),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0]);

    if value.is_some() {
        row = row.push(copy_icon_btn(value_text.to_owned()));
    }

    row.into()
}

fn fetch_hostname() -> Result<SystemValue, String> {
    let val = std::env::var("COMPUTERNAME").map_err(|e| e.to_string())?;
    Ok(SystemValue::Text(val))
}
fn fetch_username() -> Result<SystemValue, String> {
    let val = std::env::var("USERNAME").map_err(|e| e.to_string())?;
    Ok(SystemValue::Text(val))
}
fn fetch_cpu() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpus: Vec<Cpu> = sys.cpus().iter().map(|c| Cpu::from(c)).collect();

    Ok(SystemValue::Cpus(cpus))
}
fn fetch_ram() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_memory();

    Ok(SystemValue::Memory {
        total_bytes: sys.total_memory(),
        free_bytes: sys.free_memory(),
        used_bytes: sys.free_memory(),
    })
}
