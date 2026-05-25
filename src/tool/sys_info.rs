use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use iced::{
    Alignment, Length,
    widget::{self, button, row, space, text},
};
use sysinfo::System;

use super::*;
use crate::Message;

#[derive(Debug, Clone)]
pub struct Cpu {
    name: String,
    brand: String,
    vendor_id: String,
    frequency: u64,
    usage: f32,
}

impl From<&sysinfo::Cpu> for Cpu {
    fn from(value: &sysinfo::Cpu) -> Self {
        Self {
            vendor_id: value.vendor_id().trim().to_owned(),
            name: value.name().trim().to_owned(),
            brand: value.brand().trim().to_owned(),
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

fn write_bytes(w: &mut impl fmt::Write, bytes: u64) -> fmt::Result {
    const KB: u64 = 1024;
    const MB: u64 = KB * KB;
    const GB: u64 = KB * KB * KB;

    match bytes {
        GB.. => write!(w, "{:.1} GB", bytes as f64 / GB as f64),
        MB.. => write!(w, "{:.1} MB", bytes as f64 / MB as f64),
        KB.. => write!(w, "{:.1} KB", bytes as f64 / KB as f64),
        _ => write!(w, "{} B", bytes),
    }
}

impl SystemValue {
    fn display(&self) -> String {
        match self {
            SystemValue::Text(s) => s.clone(),
            SystemValue::Cpus(cpus) => {
                let mut s = String::new();
                let first = cpus.first();
                let brand = first.map(|c| c.brand.as_str()).unwrap_or("Unknown");
                let cores = cpus.len();
                let freq = first.map(|c| c.frequency).unwrap_or(0);
                write!(s, "{brand} ({cores} cores, {freq} MHz)").unwrap();
                s
            }
            SystemValue::Memory {
                total_bytes,
                free_bytes,
                used_bytes,
            } => {
                let mut s = String::new();
                write_bytes(&mut s, *used_bytes).unwrap();
                s.push_str(" used / ");
                write_bytes(&mut s, *total_bytes).unwrap();
                s.push_str(" total (");
                write_bytes(&mut s, *free_bytes).unwrap();
                s.push_str(" free)");
                s
            }
        }
    }
}

static TASKS: &[(&str, fn() -> Result<SystemValue, String>)] = &[
    ("OS Version", fetch_os),
    ("Hostname", fetch_hostname),
    ("Username", fetch_username),
    ("CPU", fetch_cpu),
    ("RAM", fetch_ram),
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
                for v in self.info.values_mut() {
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

        for (key, _) in TASKS {
            let value = &self.info[key];
            rows = rows.push(info_row(key, value))
        }

        // for (k, v) in &self.info {
        //     rows = rows.push(info_row(k, v));
        // }

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
                                    Some(format!("{k}: {}", val.display()))
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
    let label = text(key.to_string())
        .size(15)
        .width(Length::Fixed(160.0))
        .color(rgb8(160, 160, 160));

    let content: Element<'a, crate::Message> = match value {
        None => text("Loading...")
            .size(15)
            .color(rgb8(140, 140, 140))
            .into(),
        Some(Err(e)) => text(format!("ERROR: {e}"))
            .size(15)
            .color(rgb8(220, 10, 10))
            .into(),
        Some(Ok(v)) => {
            let val = v.display();
            row![
                text(val.clone())
                    .size(15)
                    .width(Length::Fill)
                    .color(rgb8(200, 200, 200)),
                copy_icon_btn(val),
            ]
            .into()
        }
    };

    row![label, content]
        .align_y(Alignment::Center)
        .padding([5, 0])
        .into()
}

fn fetch_hostname() -> Result<SystemValue, String> {
    System::host_name()
        .map(SystemValue::Text)
        .ok_or_else(|| "unavailable".to_string())
}

fn fetch_username() -> Result<SystemValue, String> {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .map(SystemValue::Text)
        .map_err(|e| e.to_string())
}

fn fetch_cpu() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    let cpus = sys.cpus().iter().map(Cpu::from).collect();
    Ok(SystemValue::Cpus(cpus))
}

fn fetch_ram() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_memory();
    Ok(SystemValue::Memory {
        total_bytes: sys.total_memory(),
        free_bytes: sys.free_memory(),
        used_bytes: sys.used_memory(),
    })
}

fn fetch_os() -> Result<SystemValue, String> {
    let os_ver = System::long_os_version().ok_or("unknown OS".to_owned())?;
    let kernel_ver = System::kernel_long_version(); //.ok_or("unknown kernel version".to_owned())?;

    let info = format!("{os_ver} ({kernel_ver})");

    Ok(SystemValue::Text(info))
}
