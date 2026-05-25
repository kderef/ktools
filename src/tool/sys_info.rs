use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use iced::{
    Alignment, Length,
    widget::{self, button, progress_bar, row, space, text},
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
    System {
        name: String,
        version: String,
    },
    Cpus(Vec<Cpu>),
    Memory {
        total_bytes: u64,
        free_bytes: u64,
        used_bytes: u64,
    },
    Disks(Vec<Disk>),
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub mount: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
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

fn bytes_string(bytes: u64) -> String {
    let mut s = String::new();
    write_bytes(&mut s, bytes).unwrap();
    s
}

impl SystemValue {
    fn display(&self) -> String {
        match self {
            SystemValue::Text(s) => s.clone(),
            SystemValue::System { name, version } => format!("{name} ({version})"),
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
            SystemValue::Disks(disks) => disks
                .iter()
                .map(|d| {
                    let mut s = String::new();
                    write!(s, "{} ({}): ", d.name, d.mount).unwrap();
                    write_bytes(&mut s, d.used_bytes).unwrap();
                    s.push_str(" / ");
                    write_bytes(&mut s, d.total_bytes).unwrap();
                    s
                })
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

static TASKS: &[(&str, fn() -> Result<SystemValue, String>)] = &[
    ("OS Version", fetch_os),
    ("Hostname", fetch_hostname),
    ("Username", fetch_username),
    ("CPU", fetch_cpu),
    ("RAM", fetch_ram),
    ("Disks", fetch_disks),
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
        let mut rows = widget::column![].spacing(2);

        for (key, _) in TASKS {
            let value = &self.info[key];
            rows = rows.push(info_row(key, value));
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
                        let text = TASKS
                            .iter()
                            .filter_map(|(k, _)| {
                                if let Some(Ok(val)) = &self.info[k] {
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
    value: &'a Option<Result<SystemValue, String>>,
) -> Element<'a, crate::Message> {
    let label = text(key.to_string())
        .size(14)
        .width(Length::Fixed(120.0))
        .color(rgb8(140, 140, 140));

    let content: Element<'a, crate::Message> = match value {
        None => text("Loading...")
            .size(14)
            .color(rgb8(100, 100, 100))
            .into(),
        Some(Err(e)) => text(format!("ERROR: {e}"))
            .size(14)
            .color(rgb8(220, 10, 10))
            .into(),
        Some(Ok(v)) => value_widget(v),
    };

    row![label, content]
        .align_y(Alignment::Center)
        .padding([6, 0])
        .into()
}

fn value_widget<'a>(value: &'a SystemValue) -> Element<'a, crate::Message> {
    match value {
        SystemValue::Text(s) => row![
            text(s.clone())
                .size(14)
                .width(Length::Fill)
                .color(rgb8(220, 220, 220)),
            copy_icon_btn(s.clone()),
        ]
        .align_y(Alignment::Center)
        .into(),
        sys @ SystemValue::System { name, version } => row![
            text(name).size(14).color(rgb8(220, 220, 220)),
            space().width(8),
            text(format!("({version})"))
                .size(14)
                .color(rgb8(160, 160, 160))
                .width(Length::Fill),
            copy_icon_btn(sys.display())
        ]
        .width(Length::Fill)
        .into(),

        SystemValue::Cpus(cpus) => {
            let first = cpus.first();
            let brand = first.map(|c| c.brand.as_str()).unwrap_or("Unknown");
            let cores = cpus.len();
            let freq = first.map(|c| c.frequency).unwrap_or(0);

            let copy_val = format!("{brand} ({cores} cores, {freq} MHz)");

            let brand_text = text(brand).size(14).color(rgb8(180, 210, 255)); // blue-ish for hardware

            let cores_text = text(format!(" · {cores} cores"))
                .size(14)
                .color(rgb8(160, 160, 160));

            let freq_text = text(format!(" · {freq} MHz"))
                .size(14)
                .color(rgb8(120, 200, 150)); // green for frequency

            row![
                brand_text,
                cores_text,
                freq_text,
                space().width(Length::Fill),
                copy_icon_btn(copy_val),
            ]
            .align_y(Alignment::Center)
            .into()
        }

        SystemValue::Memory {
            total_bytes,
            used_bytes,
            free_bytes,
        } => {
            let ratio = *used_bytes as f32 / (*total_bytes).max(1) as f32;

            let bar_color = if ratio > 0.85 {
                rgb8(220, 60, 60) // red when nearly full
            } else if ratio > 0.60 {
                rgb8(220, 160, 40) // amber when moderately used
            } else {
                rgb8(80, 180, 100) // green when healthy
            };

            let used_str = bytes_string(*used_bytes);
            let total_str = bytes_string(*total_bytes);
            let free_str = bytes_string(*free_bytes);
            let copy_val = format!("{used_str} used / {total_str} total ({free_str} free)");

            let bar = widget::container(progress_bar(0.0..=1.0, ratio).style(move |_theme| {
                iced::widget::progress_bar::Style {
                    background: iced::Background::Color(rgb8(50, 50, 50)),
                    bar: iced::Background::Color(bar_color),
                    border: iced::Border::default(),
                }
            }))
            .width(120)
            .height(8);

            let used_text = text(used_str).size(14).color(bar_color);

            let total_text = text(format!(" / {total_str}"))
                .size(14)
                .color(rgb8(160, 160, 160));

            let free_text = text(format!("  ({free_str} free)"))
                .size(13)
                .color(rgb8(110, 110, 110));

            row![
                bar,
                space().width(10),
                used_text,
                total_text,
                free_text,
                space().width(Length::Fill),
                copy_icon_btn(copy_val),
            ]
            .align_y(Alignment::Center)
            .into()
        }

        SystemValue::Disks(disks) => {
            let mut col = widget::column![].spacing(6);

            for disk in disks {
                let ratio = disk.used_bytes as f32 / disk.total_bytes.max(1) as f32;

                let bar_color = if ratio > 0.90 {
                    rgb8(220, 60, 60)
                } else if ratio > 0.70 {
                    rgb8(220, 160, 40)
                } else {
                    rgb8(80, 180, 100)
                };

                let used_str = bytes_string(disk.used_bytes);
                let total_str = bytes_string(disk.total_bytes);
                let copy_val = format!("{} ({}): {used_str} / {total_str}", disk.name, disk.mount);

                let bar = widget::container(progress_bar(0.0..=1.0, ratio).style(move |_theme| {
                    iced::widget::progress_bar::Style {
                        background: iced::Background::Color(rgb8(50, 50, 50)),
                        bar: iced::Background::Color(bar_color),
                        border: iced::Border::default(),
                    }
                }))
                .width(120)
                .height(8);

                let name_text = text(disk.name.clone()).size(14).color(rgb8(180, 210, 255)); // blue for device name

                let mount_text = text(format!(" ({})", disk.mount))
                    .size(13)
                    .color(rgb8(120, 120, 120));

                let usage_text = text(format!(" {used_str} / {total_str}"))
                    .size(14)
                    .color(bar_color);

                col = col.push(
                    row![
                        bar,
                        space().width(10),
                        name_text,
                        mount_text,
                        usage_text,
                        space().width(Length::Fill),
                        copy_icon_btn(copy_val),
                    ]
                    .align_y(Alignment::Center),
                );
            }

            col.into()
        }
    }
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
    Ok(SystemValue::System {
        name: System::long_os_version().ok_or("unknown OS type".to_owned())?,
        version: System::kernel_long_version(),
    })
}

fn fetch_disks() -> Result<SystemValue, String> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    println!("{disks:#?}");

    let disks = disks
        .iter()
        .map(|d| Disk {
            name: d.name().to_string_lossy().to_string(),
            mount: d.mount_point().to_string_lossy().to_string(),
            total_bytes: d.total_space(),
            free_bytes: d.available_space(),
            used_bytes: d.total_space() - d.available_space(),
        })
        .collect();
    Ok(SystemValue::Disks(disks))
}
