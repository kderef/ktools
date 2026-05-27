use crate::Message;

use super::*;
use iced::{
    Alignment, Background, Length, Theme,
    widget::{self, button, row, rule, space, text},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
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
            Self::Night => iced::Theme::TokyoNightStorm,
        }
    }
}

impl ThemeSetting {
    fn label(&self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::Night => "Night",
        }
    }

    fn all() -> &'static [ThemeSetting] {
        &[Self::Dark, Self::Light, Self::Night]
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeSetting,
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
    fn name(&self) -> &str {
        "Settings"
    }
    fn hidden(&self) -> bool {
        true
    }
    fn icon(&self) -> Text<'_> {
        icon_font::settings_gear()
    }
    fn background(&self, _theme: &Theme) -> Color {
        rgb8(0, 100, 180)
    }
    fn save(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(self).unwrap())
    }
    fn load(&mut self, data: serde_json::Value) {
        if let Ok(s) = serde_json::from_value(data) {
            *self = s;
        }
    }
    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            crate::Message::SetTheme(theme) => {
                self.theme = theme;
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let theme_buttons = ThemeSetting::all()
            .iter()
            .fold(row![].spacing(8), |row, &t| {
                let active = t == self.theme;

                row.push(
                    button(text(t.label()).size(14).center())
                        .on_press(Message::SetTheme(t))
                        .width(Length::Fixed(70.0))
                        .style(move |theme: &Theme, status| {
                            let palette = theme.extended_palette();
                            button::Style {
                                background: Some(Background::Color(if active {
                                    palette.primary.strong.color
                                } else {
                                    match status {
                                        button::Status::Hovered => palette.background.weak.color,
                                        _ => palette.background.strong.color,
                                    }
                                })),
                                border: iced::Border {
                                    color: if active {
                                        palette.primary.base.color
                                    } else {
                                        palette.background.strong.color
                                    },
                                    width: 1.0,
                                    radius: 6.0.into(),
                                },
                                text_color: if active {
                                    palette.primary.strong.text
                                } else {
                                    palette.background.base.text
                                },
                                ..Default::default()
                            }
                        }),
                )
            });

        let rows = widget::column![
            section_header("Appearance"),
            setting_row("Theme", theme_buttons),
            space().height(16),
            section_header("About"),
            setting_row(
                "Version",
                text(env!("CARGO_PKG_VERSION")).size(15).style(text::base)
            ),
            setting_row("Author", text("Kian Heitkamp").size(15).style(text::base)),
        ]
        .spacing(4);

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
