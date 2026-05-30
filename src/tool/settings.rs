use crate::{Message, define_themes};

use super::*;
use iced::{
    Alignment, Background, Length, Theme,
    widget::{self, button, container, row, rule, space, text},
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

    pub tool_order: Vec<String>,
}
impl Default for Settings {
    fn default() -> Self {
        let tools = crate::tool::all();
        Self {
            theme: ThemeSetting::default(),
            tool_order: tools.iter().map(|t| t.name().to_string()).collect(),
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
    fn name(&self) -> &str {
        "Settings"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::settings_gear()
    }
    fn background(&self, _theme: &Theme) -> Color {
        rgb8(0, 100, 180)
    }
    fn save(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
    fn load(&mut self, data: serde_json::Value) {
        if let Ok(mut s) = serde_json::from_value::<Self>(data) {
            let tools = std::mem::take(&mut self.tools);

            let mut invalid_list = false;

            // Unknown element in list
            for name in &s.tool_order {
                if tools.iter().find(|t| t.name() == name.as_str()).is_none() {
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
                s.tool_order = tools.iter().map(|t| t.name().to_string()).collect();
            }

            *self = s;

            self.tools = tools;
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTheme(theme) => {
                self.theme = theme;
            }
            Message::ResetToolOrder => {
                self.tool_order = self.tools.iter().map(|t| t.name().to_string()).collect();
            }

            Message::MoveToolUp(i) => {
                if i > 0 {
                    self.tool_order.swap(i, i - 1);
                }
            }
            Message::MoveToolDown(i) => {
                if i + 1 < self.tool_order.len() {
                    self.tool_order.swap(i, i + 1);
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

        let reset_order_btn = button(text("default order").size(16).center())
            .on_press(Message::ResetToolOrder)
            .width(300);

        let rows = widget::column![
            section_header("Appearance"),
            setting_row("Theme", theme_buttons),
            space().height(16),
            //
            section_header("Tool Order"),
            self.tool_order_list(),
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
            setting_row("Source Code", source_link()),
            setting_row("Version", app_version()),
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

impl Settings {
    fn tool_order_list<'a>(&'a self) -> Element<'a, Message> {
        let rows = self.tool_order.iter().enumerate().map(|(i, name)| {
            let is_first = i == 0;
            let is_last = i == self.tool_order.len() - 1;

            let tool = self.tools.iter().find(|t| t.name() == name.as_str());

            let icon_and_name: Element<'_, Message> = if let Some(tool) = tool {
                let bg = tool.background(&self.theme.into()); // use a neutral theme for preview
                row![
                    container(tool.icon().size(16))
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
                    text(name.clone()).size(14),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .into()
            } else {
                text(name.clone()).size(14).into()
            };

            row![
                icon_and_name,
                space().width(Length::Fill),
                button(icon_font::arrow_up().size(12))
                    .on_press_maybe((!is_first).then_some(Message::MoveToolUp(i)))
                    .padding([2, 6]),
                button(icon_font::arrow_down().size(12))
                    .on_press_maybe((!is_last).then_some(Message::MoveToolDown(i)))
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
