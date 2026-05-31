use std::fmt;
use std::{collections::HashMap, os::windows::process::CommandExt};

use iced::{
    Alignment, Background, Border, Length, Theme,
    widget::{self, button, progress_bar, row, space, text},
};
use sysinfo::System;

use super::*;
use crate::Message;

/// Value returned from fetching info
/// Example: `fetch_hostname()` could return `SystemValue::Text("my computer")`
#[derive(Debug, Clone)]
pub enum SystemValue {
    Text(String),
    System {
        name: String,
        version: String,
        arch: String,
    },
    Cpu {
        brand: String,
        /// Frequency in GHz
        frequency: f32,
        cores: usize,
    },
    Memory {
        total: Bytes,
        free: Bytes,
        used: Bytes,
    },
    Disks(Vec<Disk>),
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub mount: String,
    pub total: Bytes,
    pub free: Bytes,
    pub used: Bytes,
}

/// newtype to make it easy to print bytes (automatically formats as GB, MB, etc)
#[derive(Debug, Clone)]
pub struct Bytes(u64);

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;
        const KB: u64 = 1024;

        match self.0 {
            GB.. => write!(f, "{:.1} GB", self.0 as f64 / GB as f64),
            MB.. => write!(f, "{:.1} MB", self.0 as f64 / MB as f64),
            KB.. => write!(f, "{:.1} KB", self.0 as f64 / KB as f64),
            _ => write!(f, "{} B", self.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessOpen {
    ConfigPanel,
    Printers,
    AdminTools,
    Features,
    ComputerManagement,
    PowerOptions,
    DeviceManager,
    InstalledApps,
}

impl ProcessOpen {
    pub const fn command(self) -> &'static [&'static str] {
        match self {
            Self::ConfigPanel => &["control"],
            Self::Printers => &["explorer", "ms-settings:printers"],
            Self::AdminTools => &["control", "/name", "Microsoft.AdministrativeTools"],
            Self::Features => &["rundll32", "shell32.dll,Control_RunDLL", "appwiz.cpl,,2"],
            // Self::Features => &["control", "appwiz.cpl"],
            Self::ComputerManagement => &["cmd", "/C", "compmgmt.msc"],
            Self::PowerOptions => &["explorer", "ms-settings:powersleep"],
            Self::DeviceManager => &["cmd", "/C", "devmgmt.msc"],
            Self::InstalledApps => &["explorer", "ms-settings:appsfeatures"],
        }
    }
    pub fn icon(self) -> Text<'static> {
        match self {
            Self::ConfigPanel => icon_font::settings_gear(),
            Self::Printers => icon_font::preview(),
            Self::AdminTools => icon_font::account(),
            Self::Features => icon_font::tools(),
            Self::ComputerManagement => icon_font::vm(),
            Self::PowerOptions => icon_font::plug(),
            Self::DeviceManager => icon_font::server(),
            Self::InstalledApps => icon_font::multiple_windows(),
        }
    }
}

impl ToString for SystemValue {
    fn to_string(&self) -> String {
        match self {
            SystemValue::Text(s) => s.clone(),
            SystemValue::System {
                name,
                version,
                arch,
            } => format!("{name} {arch} ({version})"),
            SystemValue::Cpu {
                brand,
                frequency,
                cores,
            } => {
                format!("{brand} ({cores}, {frequency:.2} GHz)")
            }
            SystemValue::Memory {
                total: total_bytes,
                free: free_bytes,
                used: used_bytes,
            } => {
                format!("{used_bytes} used / {total_bytes} total ({free_bytes} free)")
            }
            SystemValue::Disks(disks) => disks
                .iter()
                .map(|d| format!("{} ({}): {} / {}", d.name, d.mount, d.used, d.total))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

impl SystemValue {}

/// Tasks that are performed simultaneously in their own `Task`'s
/// The name is used for indexing into a `HashMap`, as well as displaying.
static TASKS: &[(&str, fn() -> Result<SystemValue, String>)] = &[
    ("System", fetch_os),
    ("Hostname", fetch_hostname),
    ("Username", fetch_username),
    ("CPU", fetch_cpu),
    ("Graphics Card", fetch_graphics_card),
    ("RAM", fetch_ram),
    ("Disks", fetch_disks),
];

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
    fn name(&self) -> &str {
        "System Information"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::vm()
    }
    fn background(&self, theme: &Theme) -> Color {
        // rgb8(66, 123, 88)
        let pal = theme.extended_palette();

        if pal.is_dark {
            pal.warning.weak.color
        } else {
            pal.warning.strong.color
        }
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        // Launch tasks for each of the TASKS
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
                    proc_button("Configuration Panel", ProcessOpen::ConfigPanel),
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
    let label = text(key.to_string()).size(14).width(Length::Fixed(120.0));
    // .color(rgb8(140, 140, 140));

    let content: Element<'a, crate::Message> = match value {
        None => text("Loading...").size(14).style(text::secondary).into(),
        Some(Err(e)) => text(format!("ERROR: {e}"))
            .size(14)
            .style(text::danger)
            .into(),
        Some(Ok(v)) => value_widget(v),
    };

    row![label, content]
        .align_y(Alignment::Center)
        .padding([6, 0])
        .into()
}

fn value_widget<'a>(value: &'a SystemValue) -> Element<'a, Message> {
    match value {
        SystemValue::Text(s) => row![
            text(s.clone()).size(14).width(Length::Fill),
            // .color(rgb8(220, 220, 220)),
            copy_icon_btn(s.clone()),
        ]
        .align_y(Alignment::Center)
        .into(),
        sys @ SystemValue::System {
            name,
            version,
            arch,
        } => row![
            text(name).size(14),
            space().width(8),
            text(arch).style(text::secondary).size(14),
            space().width(8),
            text(format!("( {version} )"))
                .size(14)
                .style(|theme: &Theme| {
                    text::Style {
                        color: {
                            let mut color = theme.palette().text;
                            color.a = 0.8;
                            Some(color)
                        },
                    }
                })
                .width(Length::Fill),
            copy_icon_btn(sys.to_string())
        ]
        .width(Length::Fill)
        .into(),

        SystemValue::Cpu {
            brand,
            frequency,
            cores,
        } => {
            let brand_text = text(brand).size(14).style(text::primary);

            let cores_text = text(format!(" · {cores} cores")).size(14);

            let freq_text =
                text(format!(" · {frequency:.2} GHz"))
                    .size(14)
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().success.strong.color),
                    });

            row![
                brand_text,
                cores_text,
                freq_text,
                space().width(Length::Fill),
                copy_icon_btn(value.to_string()),
            ]
            .align_y(Alignment::Center)
            .into()
        }

        SystemValue::Memory { total, used, free } => {
            let ratio = used.0 as f32 / total.0.max(1) as f32;

            // Red, amber or green for status
            let bar_color = match ratio {
                0.85.. => rgb8(220, 60, 60),
                0.60.. => rgb8(220, 160, 40),
                _ => rgb8(80, 180, 100),
            };

            let bar = widget::container(progress_bar(0.0..=1.0, ratio).style(move |_theme| {
                iced::widget::progress_bar::Style {
                    background: iced::Background::Color(rgb8(50, 50, 50)),
                    bar: Background::Color(bar_color),
                    border: Border::default(),
                }
            }))
            .width(120)
            .height(8);

            let used_text = text(format!("{used}")).size(14).color(bar_color);

            let total_text = text(format!(" / {total}")).size(14).style(text::secondary);

            let free_text = text(format!("  ({free} free)"))
                .size(13)
                .color(rgb8(110, 110, 110));

            row![
                bar,
                space().width(10),
                used_text,
                total_text,
                free_text,
                space().width(Length::Fill),
                copy_icon_btn(value.to_string()),
            ]
            .align_y(Alignment::Center)
            .into()
        }

        SystemValue::Disks(disks) => {
            let mut col = widget::column![].spacing(6);

            for disk in disks {
                let Disk {
                    name,
                    mount,
                    total,
                    free: _,
                    used,
                } = disk;

                let ratio = used.0 as f32 / total.0.max(1) as f32;

                // Color changes depending on how full disk is
                let bar_color = match ratio {
                    0.90.. => rgb8(220, 60, 60),
                    0.70.. => rgb8(220, 160, 40),
                    _ => rgb8(80, 180, 100),
                };

                let bar = widget::container(progress_bar(0.0..=1.0, ratio).style(move |_theme| {
                    iced::widget::progress_bar::Style {
                        background: iced::Background::Color(rgb8(50, 50, 50)),
                        bar: Background::Color(bar_color),
                        border: Border::default(),
                    }
                }))
                .width(120)
                .height(8);

                let name_text = text(name.clone()).size(14).style(text::primary); // .color(rgb8(180, 210, 255)); // blue for device name

                let mount_text = text(format!(" ({mount})"))
                    .size(13)
                    .color(rgb8(120, 120, 120));

                let usage_text = text(format!(" {used} / {total}")).size(14).color(bar_color);

                col = col.push(
                    row![
                        bar,
                        space().width(10),
                        name_text,
                        mount_text,
                        usage_text,
                        space().width(Length::Fill),
                        copy_icon_btn(value.to_string()),
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
        .ok_or("unavailable".to_string())
}

fn fetch_username() -> Result<SystemValue, String> {
    std::env::var("USERNAME")
        .or(std::env::var("USER"))
        .map(SystemValue::Text)
        .map_err(|e| e.to_string())
}

fn fetch_cpu() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let cpu = cpus.first().ok_or("No cpu was found".to_owned())?;

    Ok(SystemValue::Cpu {
        brand: cpu.brand().trim().to_owned(),
        frequency: cpu.frequency() as f32 / 1000.0,
        cores: cpus.len(),
    })
}

fn fetch_ram() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_memory();

    Ok(SystemValue::Memory {
        total: Bytes(sys.total_memory()),
        free: Bytes(sys.free_memory()),
        used: Bytes(sys.used_memory()),
    })
}

fn fetch_os() -> Result<SystemValue, String> {
    Ok(SystemValue::System {
        name: System::long_os_version().ok_or("unknown OS type".to_owned())?,
        version: System::kernel_long_version(),
        arch: System::cpu_arch(),
    })
}

fn fetch_disks() -> Result<SystemValue, String> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    let disks = disks
        .iter()
        .map(|d| Disk {
            name: d.name().to_string_lossy().to_string(),
            mount: d.mount_point().to_string_lossy().to_string(),
            total: Bytes(d.total_space()),
            free: Bytes(d.available_space()),
            used: Bytes(d.total_space() - d.available_space()),
        })
        .collect();

    Ok(SystemValue::Disks(disks))
}

fn fetch_graphics_card() -> Result<SystemValue, String> {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIFactory};

        let factory: IDXGIFactory = CreateDXGIFactory().map_err(|e| e.to_string())?;

        let adapter = factory.EnumAdapters(0).map_err(|e| e.to_string())?;

        let desc = adapter.GetDesc().map_err(|e| e.to_string())?;

        let name = String::from_utf16_lossy(&desc.Description);

        Ok(SystemValue::Text(name.trim_end_matches('\0').to_string()))
    }
}
