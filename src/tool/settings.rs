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
}
impl Default for Settings {
    fn default() -> Self {
        let tools = crate::tool::all();
        Self {
            theme: ThemeSetting::default(),
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
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTheme(theme) => {
                self.theme = theme;
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
            setting_row("Source Code", source_link()),
            setting_row("License", license_link()),
        ]
        .spacing(4);

        let container = content_container(rows).padding(12).height(Length::Fill);

        let col = widget::column![container];

        col.height(Length::Fill).padding(12).into()
    }
}
