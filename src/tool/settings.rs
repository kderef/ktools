use iced::{
    Alignment, Length,
    widget::{self, button, row, space, text},
};
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
        let mut rows = widget::column![];

        let container = content_container(rows).padding(12).height(Length::Fill);
        let go_back = go_back_button(13);
        let title = title_text(self);

        let col = widget::column![
            widget::row![go_back, space().width(16), title.align_y(Alignment::Center)]
                .align_y(Alignment::Center),
            space().height(10),
            container
        ];

        col.height(Length::Fill).padding(12).into()
    }
}
