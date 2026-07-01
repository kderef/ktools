//! settings: the "settings" page of the app. Contains global settings relevant to all tools and the app

use std::{env, fs, io, path::PathBuf};

use crate::{Message, debug, define_themes, download};
use download::Progress;

use super::*;
use iced::{
    Alignment, Length, Padding,
    widget::{self, Row, button, container, pick_list, progress_bar, row, rule, space, text},
};
use serde::{Deserialize, Serialize};

define_themes! {
    ThemeSetting {
        Dark => iced::Theme::Dark,
        Light => iced::Theme::Light,
        Night => iced::Theme::TokyoNight,
        Solarized => iced::Theme::SolarizedDark
    }
}

#[derive(Default)]
pub struct Settings {
    /// This is not actually used by the app, it's here to be shown in the picker for the theme.
    theme_copy: ThemeSetting,
    latest_git_tag: Option<Result<String, String>>,

    download_progress: u32,
    downloading: bool,
    download_result: Option<Result<Vec<u8>, download::DownloadError>>,
}

fn section_header<'a>(label: &'a str) -> Element<'a, Message> {
    widget::column![text(label).size(13).style(text::base), rule::horizontal(1),]
        .spacing(4)
        .into()
}

fn setting_row<'a>(
    label: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .style(text::primary),
        content.into(),
    ]
    .align_y(Alignment::Center)
    .padding([6, 0])
    .into()
}

impl Tool for Settings {
    fn name(&self) -> &'static str {
        "Settings"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::settings_gear()
    }
    fn sidebar_position(&self) -> SidebarPosition {
        SidebarPosition::Bottom
    }

    fn load_data(&mut self) -> Task<crate::Message> {
        Task::perform(
            async {
                tokio::task::spawn_blocking(get_latest_build_tag)
                    .await
                    .unwrap()
            },
            Message::FetchedLatestGitTag,
        )
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::none()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FetchedLatestGitTag(result) => {
                self.latest_git_tag = Some(result);
            }
            Message::SetTheme(theme) => {
                self.theme_copy = theme;
            }

            Message::DownloadStarted(_) => {
                self.download_progress = 0;
                self.downloading = true;
                self.download_result = None;
            }

            Message::DownloadProgress(_, progress) => match progress {
                Progress::Finished => {
                    self.downloading = false;
                    self.download_result = None;
                }
                Progress::Downloading(completion) => {
                    self.download_progress = completion;
                }
            },

            Message::DownloadFinished(_, result) => {
                self.download_progress = 100;
                self.downloading = false;

                match result {
                    Ok(bytes) => {
                        match apply_update(&bytes) {
                            Err(e) => {
                                debug!("[SELF-UPDATE] failed to apply: {e}");
                                self.download_result = Some(Err(e.to_string()));
                            }
                            Ok(_) => {
                                // ask user to restart
                                return Task::done(Message::ShowSelfUpdateMessage);
                            }
                        }
                        // NOTE: on success apply_update() never returns (process::exit(0)),
                        // so we only ever reach here on failure.
                    }
                    Err(e) => {
                        debug!("[SELF-UPDATE] download failed: {e}");
                        self.download_result = Some(Err(e));
                    }
                }
            }

            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let reset_button = button("RESET ALL SETTINGS")
            .style(button::danger)
            .on_press(Message::ResetAllSettings);

        let theme_picker = pick_list(
            ThemeSetting::all(),
            Some(self.theme_copy),
            Message::SetTheme,
        );

        let rows = widget::column![
            section_header("Appearance"),
            setting_row("Theme", theme_picker),
            space().height(16),
            //
            space().height(16),
            //
            section_header("Tool Settings"),
            setting_row("All Tools", reset_button),
            space().height(16),
            //
            section_header("About"),
            setting_row(
                "Developer",
                text("Kian Heitkamp").size(15).style(text::base)
            ),
            setting_row("Version", app_version()),
            setting_row("Latest Version", self.app_latest_version()),
            setting_row("Source Code", source_link()),
            setting_row("License", license_link()),
        ]
        .spacing(4);

        let container = content_container(rows).padding(12).height(Length::Fill);

        let col = widget::column![container];

        col.height(Length::Fill).padding(12).into()
    }
}

pub fn get_latest_build_tag() -> Result<String, String> {
    const SOURCE_LINK: &str = env!("CARGO_PKG_REPOSITORY");
    let repo_name = SOURCE_LINK.splitn(4, '/').last().unwrap();
    let latest_url = format!("https://github.com/{repo_name}/releases/latest");

    let response = minreq::get(&latest_url)
        .with_header("User-Agent", "KTools")
        .with_timeout(5)
        .with_follow_redirects(false)
        .send()
        .map_err(|e| e.to_string())?;

    // GitHub responds with a 302 whose Location header looks like:
    // https://github.com/{owner}/{repo}/releases/tag/{tag}
    let location = response
        .headers
        .get("location")
        .ok_or_else(|| "GitHub did not return a redirect".to_string())?;

    location
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| "Could not parse tag from redirect URL".to_string())
}

impl Settings {
    fn app_latest_version(&self) -> Row<'_, Message> {
        let ver_text = match &self.latest_git_tag {
            None => text("loading...").style(text::secondary),
            Some(Ok(s)) => text(s.strip_prefix('v').unwrap_or(&s)),
            Some(Err(_)) => text("unknown").style(text::secondary),
        };

        let current_version = concat!("v", env!("CARGO_PKG_VERSION"));

        let latest_release_url = match &self.latest_git_tag {
            // If the version is already latest, we do not need the button.
            Some(Ok(tag)) if tag == current_version => None,

            // A release was found and is not the same as the app
            Some(Ok(tag)) => Some(format!(
                "{}/releases/download/{tag}/ktools.exe",
                env!("CARGO_PKG_REPOSITORY")
            )),
            _ => None,
        };

        // This will change depending on what the status of the update is
        let update_widget: Element<'_, Message> = if self.downloading {
            let download_progress = format!("{}%", self.download_progress as i64);

            let row = widget::row![
                progress_bar(0.0..=100.0, self.download_progress as f32)
                    .girth(Length::Fill)
                    .length(250),
                space().width(5),
                text(download_progress).size(15).height(Length::Shrink)
            ];

            container(row).center_y(Length::Fill).into()
        } else if let Some(Err(e)) = &self.download_result {
            let retry_url = latest_release_url.clone();

            widget::row![
                text(format!("Update failed: {e}"))
                    .size(14)
                    .style(text::danger),
                space().width(8),
                button("Retry")
                    .on_press_maybe(retry_url.map(Message::DownloadStart))
                    .padding(Padding {
                        top: 1.0,
                        right: 4.0,
                        bottom: 1.0,
                        left: 4.0,
                    }),
            ]
            .align_y(Alignment::Center)
            .into()
        } else {
            let button_text = match &self.latest_git_tag {
                Some(Ok(tag)) if tag == current_version => "Already up to date",
                Some(Ok(_)) => "Download new version",
                _ => "Failed to retrieve the latest version",
            };

            button(button_text)
                .on_press_maybe(latest_release_url.map(Message::DownloadStart))
                .padding(Padding {
                    top: 1.0,
                    right: 4.0,
                    bottom: 1.0,
                    left: 4.0,
                })
                .into()
        };

        row![ver_text.size(15), space().width(10), update_widget]
    }
}

fn staged_path(suffix: &str) -> io::Result<PathBuf> {
    let current_exe = env::current_exe()?;
    let stem = current_exe
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("app");
    let ext = current_exe
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let file_name = if ext.is_empty() {
        format!("{stem}{suffix}")
    } else {
        format!("{stem}{suffix}.{ext}")
    };

    Ok(current_exe.with_file_name(file_name))
}

fn apply_update(new_exe_bytes: &[u8]) -> io::Result<()> {
    let current_exe = env::current_exe()?;
    let new_path = staged_path("_new")?; // e.g. ktools_new.exe
    let old_path = staged_path("_old")?; // e.g. ktools_old.exe

    debug!("[SELF-UPDATE] writing {new_path:?}");
    fs::write(&new_path, new_exe_bytes)?;

    debug!("[SELF-UPDATE] rename {current_exe:?} -> {old_path:?}");
    fs::rename(&current_exe, &old_path)?;

    debug!("[SELF-UPDATE] rename {new_path:?} -> {current_exe:?}");
    fs::rename(&new_path, &current_exe)?;

    Ok(())
}

pub fn cleanup_old_exe() {
    let old_path = match staged_path("_old") {
        Ok(p) => p,
        Err(e) => {
            debug!("[SELF-UPDATE] could not resolve old exe path: {e}");
            return;
        }
    };

    if !old_path.exists() {
        return;
    }

    debug!("[SELF-UPDATE] cleaning up leftover {old_path:?}");

    for attempt in 0..5 {
        match fs::remove_file(&old_path) {
            Ok(()) => {
                debug!("[SELF-UPDATE] removed {old_path:?}");
                return;
            }
            Err(e) if attempt < 4 => {
                debug!("[SELF-UPDATE] remove attempt {attempt} failed: {e}, retrying");
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => {
                debug!("[SELF-UPDATE] giving up removing {old_path:?}: {e}");
            }
        }
    }
}
