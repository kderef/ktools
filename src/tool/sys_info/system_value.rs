use super::*;

/// Value returned from fetching info
/// Example: `fetch_hostname()` could return `SystemValue::Text("my computer")`
#[derive(Debug, Clone)]
pub enum SystemValue {
    Text(String),
    System {
        description_long: String,
        description_short: String,
        kernel_version: String,
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

/// wrapper to make it easy to print bytes (automatically formats as GB, MB, etc)
#[derive(Debug, Clone)]
pub struct Bytes(pub u64);

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

impl ToString for SystemValue {
    fn to_string(&self) -> String {
        match self {
            SystemValue::Text(s) => s.clone(),
            SystemValue::System {
                description_long,
                description_short: _,
                kernel_version,
                arch,
            } => format!("{description_long} {arch} ({kernel_version})"),
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

impl SystemValue {
    pub fn widget<'a>(&'a self) -> Element<'a, crate::Message> {
        match self {
            Self::Text(s) => row![
                text(s.clone()).size(14).width(Length::Fill),
                copy_icon_btn(s.clone()),
            ]
            .align_y(Alignment::Center)
            .into(),

            sys @ Self::System {
                description_long,
                description_short: _,
                kernel_version,
                arch,
            } => row![
                text(description_long).size(14),
                space().width(8),
                text(arch).style(text::secondary).size(14),
                space().width(8),
                text(format!("( {kernel_version} )"))
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

            Self::Cpu {
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
                    copy_icon_btn(self.to_string()),
                ]
                .align_y(Alignment::Center)
                .into()
            }

            Self::Memory { total, used, free } => {
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
                    copy_icon_btn(self.to_string()),
                ]
                .align_y(Alignment::Center)
                .into()
            }

            Self::Disks(disks) => {
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

                    let bar =
                        widget::container(progress_bar(0.0..=1.0, ratio).style(move |_theme| {
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
                            copy_icon_btn(self.to_string()),
                        ]
                        .align_y(Alignment::Center),
                    );
                }

                col.into()
            }
        }
    }
}
