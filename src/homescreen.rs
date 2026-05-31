//! The UI of the home page.

use iced::{
    Background, Border, Color, Element, Length,
    widget::{self, *},
};

use crate::{App, Message, base::rgb8};

pub fn tool_button_simple<'a>(
    icon: Text<'a>,
    name: &'a str,
    bg: Color,
    index: usize,
) -> Button<'a, Message> {
    let icon = icon.size(28);
    button(
        container(
            iced::widget::column![icon, text(name).size(16),]
                .align_x(iced::Alignment::Center)
                .spacing(8),
        )
        .center(Length::Fill),
    )
    .width(160)
    .height(80)
    .on_press(Message::ChooseTool(index))
    .style(move |theme: &Theme, status| {
        let alpha = match status {
            button::Status::Hovered => 0.82,
            button::Status::Pressed => 0.65,
            _ => 1.0,
        };
        let tinted = Color { a: alpha, ..bg };
        button::Style {
            snap: false,
            background: Some(Background::Color(tinted)),
            text_color: rgb8(255, 255, 255),
            border: Border {
                // color: match theme {
                //     Theme::Light => Color::from_rgba(0., 0., 0., 0.8),
                //     _ => Color::from_rgba(1., 1., 1., 0.3),
                // },
                color: theme.extended_palette().secondary.base.color,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default() // shadow: iced::Shadow {
                                 //     color: rgba(0.0, 0.0, 0.0, 0.35),
                                 //     offset: iced::Vector { x: 0.0, y: 2.0 },
                                 //     blur_radius: 6.0,
                                 // },
        }
    })
}

const PADDING: u16 = 20;

pub fn view_simple<'a>(app: &'a App) -> Element<'a, Message> {
    // The grid of Tool's
    let children = app
        .settings
        .tool_order
        .iter()
        .filter_map(|name| app.tools.iter().position(|t| t.name() == name))
        .map(|i| {
            let t = &app.tools[i];
            tool_button_simple(t.icon(), t.name(), t.background(&app.theme()), i).into()
        });

    let grid = grid(children).fluid(200).spacing(20);

    let content = container(grid).padding(PADDING);
    let view = scrollable(content);

    view.into()
}
pub fn view_advanced<'a>(app: &'a App) -> Element<'a, Message> {
    let content = widget::column![];

    let content = container(content).padding(PADDING);
    let view = scrollable(content);

    view.into()
}
