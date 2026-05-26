use super::*;
use iced::{
    Alignment, Length, Theme,
    widget::{self, button, row, rule, space, text},
};
use serde::{Deserialize, Serialize};

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

fn section_header<'a>(label: &'a str) -> Element<'a, crate::Message> {
    widget::column![text(label).size(13), rule::horizontal(1),]
        .spacing(4)
        .into()
}

fn setting_row<'a>(
    label: &'a str,
    content: impl Into<Element<'a, crate::Message>>,
) -> Element<'a, crate::Message> {
    row![
        text(label).size(15).width(Length::Fixed(160.0)),
        // .color(rgb8(180, 180, 180)),
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
    fn background(&self) -> Color {
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
    fn view(&self) -> Element<'_, crate::Message> {
        // --- Appearance ---
        let theme_buttons = ThemeSetting::all()
            .iter()
            .fold(row![].spacing(8), |row, &t| {
                let active = matches!(
                    (self.theme, t),
                    (ThemeSetting::Dark, ThemeSetting::Dark)
                        | (ThemeSetting::Light, ThemeSetting::Light)
                        | (ThemeSetting::Night, ThemeSetting::Night)
                );
                row.push(
                    button(text(t.label()).size(14).center())
                        .on_press(crate::Message::SetTheme(t))
                        .width(Length::Fixed(70.0))
                        .style(move |theme: &Theme, status| widget::button::Style {
                            background: Some(iced::Background::Color(if active {
                                theme.extended_palette().primary.strong.color
                            } else {
                                match status {
                                    button::Status::Hovered => rgb8(60, 60, 60),
                                    _ => rgb8(40, 40, 40),
                                }
                            })),
                            border: iced::Border {
                                color: if active {
                                    rgb8(0, 140, 220)
                                } else {
                                    rgb8(70, 70, 70)
                                },
                                width: 1.0,
                                radius: 6.0.into(),
                            },
                            text_color: rgb8(220, 220, 220),
                            ..Default::default()
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
                text(env!("CARGO_PKG_VERSION"))
                    .size(15)
                    .style(text::secondary)
            ),
            setting_row(
                "Author",
                text("Kian Heitkamp").size(15).style(text::secondary)
            ),
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
