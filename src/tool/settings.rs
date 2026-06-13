use crate::{Message, define_themes};

use super::*;
use iced::{
    Alignment, Length,
    widget::{self, button, pick_list, row, rule, space, text},
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

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeSetting,
    #[serde(skip)]
    tools: Vec<Box<dyn Tool>>,

    #[serde(skip)]
    latest_git_tag: Option<Result<String, String>>,
}
impl Default for Settings {
    fn default() -> Self {
        let tools = crate::tool::all();
        Self {
            theme: ThemeSetting::default(),
            latest_git_tag: None,
            tools,
        }
    }
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

    fn save(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
    fn load(&mut self, data: serde_json::Value) {
        if let Ok(s) = serde_json::from_value::<Self>(data) {
            let tools = std::mem::take(&mut self.tools);

            *self = s;

            self.tools = tools;
        }
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::perform(
            async { get_latest_build_tag() },
            Message::FetchedLatestGitTag,
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTheme(theme) => {
                self.theme = theme;
            }
            Message::FetchedLatestGitTag(result) => {
                self.latest_git_tag = Some(result);
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let reset_button = button("RESET ALL SETTINGS")
            .style(button::danger)
            .on_press(Message::ResetAllSettings);

        let theme_picker = pick_list(ThemeSetting::all(), Some(self.theme), Message::SetTheme);

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
            setting_row("Latest Version", app_latest_version(&self.latest_git_tag)),
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
    let api_url = format!("https://api.github.com/repos/{repo_name}/tags");

    let response = minreq::get(api_url)
        .with_header("User-Agent", "KTools")
        .send()
        .map_err(|e| e.to_string())?
        .into_bytes();

    let response_json: serde_json::Value =
        serde_json::from_slice(&response).map_err(|e| e.to_string())?;

    match response_json {
        serde_json::Value::Array(a) => a
            .first()
            .ok_or("No build tags found")?
            .as_object()
            .ok_or("Array member was not an object")?
            .get("name")
            .ok_or("No git name tag was found.")?
            .as_str()
            .ok_or("name tag was not a string")
            .map(|s| s.to_owned()),
        _ => Err("JSon value is not an array."),
    }
    .map_err(|e| e.to_string())
}
