use iced::widget::{self, row, space, text};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ThemeSetting {
    #[default]
    Dark,
    Light,
    Night,
}

impl Into<iced::Theme> for ThemeSetting {
    fn into(self) -> iced::Theme {
        match self {
            Self::Dark => iced::Theme::Dark,
            Self::Light => iced::Theme::Light,
            Self::Night => iced::Theme::TokyoNight,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeSetting,
}

impl Tool for Settings {
    fn name(&self) -> &str {
        "Settings"
    }
    fn hidden(&self) -> bool {
        true
    }
    fn icon(&self) -> Text<'_> {
        icon_font::settings_gear()
    }
    fn background(&self) -> Color {
        rgb8(0, 100, 180)
    }
    fn save(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(self).unwrap())
    }
    fn load(&mut self, _data: serde_json::Value) {}

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let top_row = row![
            go_back_button(13), //
            space().width(10),
            title_text(self)
        ];

        widget::column![top_row].into()
    }
}
