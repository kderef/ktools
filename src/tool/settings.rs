use crate::define_themes;

use super::*;
use crate::homescreen::HomescreenStyle;
use iced::{
    Alignment, Background, Length, Theme,
    widget::{self, Row, button, container, pick_list, row, rule, space, text},
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

#[derive(Debug, Clone)]
pub enum Message {
    SetTheme(ThemeSetting),
    SetHomescreenStyle(crate::homescreen::HomescreenStyle),
    ResetAllSettings,
    Copy(String),
    OpenURL(&'static str),
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeSetting,
    pub homescreen_style: HomescreenStyle,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeSetting::default(),
            homescreen_style: HomescreenStyle::default(),
        }
    }
}

fn section_header<'a>(label: &'a str) -> Element<'a, crate::Message> {
    widget::column![text(label).size(13).style(text::base), rule::horizontal(1),]
        .spacing(4)
        .into()
}

fn setting_row<'a, M: 'a>(label: &'a str, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
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

impl Settings {
    /*
    fn load(&mut self, data: serde_json::Value) {
        if let Ok(mut s) = serde_json::from_value::<Self>(data) {
            let tools = std::mem::take(&mut self.tools);

            let mut invalid_list = false;

            // Unknown element in list
            for name in &s.tool_order {
                if tools
                    .iter()
                    .find(|t| t.info().title == name.as_str())
                    .is_none()
                {
                    invalid_list = true;
                    break;
                }
            }

            // List is not complete
            if s.tool_order.len() != tools.len() {
                invalid_list = true;
            }

            // if invalid reset to default
            if invalid_list {
                s.tool_order = tools.iter().map(|t| t.info().title.to_string()).collect();
            }

            *self = s;

            self.tools = tools;
        }
    }
    */
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::SetTheme(theme) => {
                self.theme = theme;
            }
            Message::SetHomescreenStyle(style) => {
                self.homescreen_style = style;
            }
            _ => {}
        }
        Task::none()
    }
    pub fn view<'a>(&'a self, app: &'a crate::App) -> Element<'a, crate::Message> {
        let reset_button = button("RESET ALL SETTINGS")
            .style(button::danger)
            .on_press(crate::Message::ResetAllSettings);

        let theme_buttons: Row<'_, crate::Message> =
            ThemeSetting::all()
                .iter()
                .fold(row![].spacing(8), |row, &t| {
                    let active = t == self.theme;

                    row.push(
                        button(text(t.label()).size(14).center())
                            .on_press(crate::Message::from(Message::SetTheme(t)))
                            .width(Length::Fixed(70.0))
                            .style(move |theme: &Theme, status| {
                                let palette = theme.extended_palette();
                                button::Style {
                                    background: Some(Background::Color(if active {
                                        palette.primary.strong.color
                                    } else {
                                        match status {
                                            button::Status::Hovered => {
                                                palette.background.weak.color
                                            }
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

        let reset_order_btn = button(text("default order").size(16).center())
            .on_press(crate::Message::ResetToolOrder)
            .width(300);

        let layout_picker = pick_list(HomescreenStyle::all(), Some(self.homescreen_style), |s| {
            Message::SetHomescreenStyle(s).into()
        });

        let rows = widget::column![
            section_header("Appearance"),
            setting_row("Theme", theme_buttons),
            setting_row("Tools Layout", layout_picker),
            space().height(16),
            //
            section_header("Tool Order"),
            self.tool_order_list(app),
            space().height(8),
            reset_order_btn,
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
            setting_row("License", license_link(crate::Message::OpenURL)),
        ]
        .spacing(4);

        let container = content_container(rows).padding(12).height(Length::Fill);

        let col = widget::column![container];

        col.height(Length::Fill).padding(12).into()
    }
}

impl Settings {
    fn tool_order_list<'a>(&'a self, app: &'a crate::App) -> Element<'a, crate::Message> {
        let rows = app
            .tool_order
            .iter()
            .map(|i| (i, app.tools[*i].info().title))
            .map(|(i, name)| {
                let i = *i;
                let is_first = i == 0;
                let is_last = i == app.tool_order.len() - 1;

                let tool = app.tools.iter().find(|t| t.info().title == name);

                let icon_and_name: Element<'_, crate::Message> = if let Some(tool) = tool {
                    let info = tool.info();
                    let bg = (info.background)(&self.theme.into()); // use a neutral theme for preview
                    row![
                        container((info.icon)().size(16))
                            .style(move |_| container::Style {
                                background: Some(Background::Color(bg)),
                                text_color: Some(iced::Color::WHITE),
                                border: iced::Border {
                                    radius: 6.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .padding([4, 8]),
                        text(name).size(14),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .into()
                } else {
                    text(name).size(14).into()
                };

                row![
                    icon_and_name,
                    space().width(Length::Fill),
                    button(icon_font::arrow_up().size(12))
                        .on_press_maybe((!is_first).then_some(crate::Message::MoveToolUp(i)))
                        .padding([2, 6]),
                    button(icon_font::arrow_down().size(12))
                        .on_press_maybe((!is_last).then_some(crate::Message::MoveToolDown(i)))
                        .padding([2, 6]),
                ]
                .spacing(4)
                .width(300)
                .align_y(Alignment::Center)
                .into()
            });

        widget::column(rows).spacing(4).into()
    }
}
