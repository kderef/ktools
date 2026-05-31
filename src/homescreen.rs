//! The UI of the home page.

use iced::{
    Background, Border, Color, Element, Length,
    widget::{self, *},
};

use crate::{
    App, Message,
    base::{BOLD_DEFAULT, rgb8},
    tool::{Category, Tool, settings::HomescreenStyle},
};

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

fn tool_small_button<'a>(t: &'a dyn Tool, index: usize) -> Button<'a, Message> {
    use button::Status;

    button(widget::row![t.icon(), space().width(5), t.name()])
        .on_press(Message::ChooseTool(index))
        .padding(0)
        .style(|theme, status| {
            let ex = theme.extended_palette();
            let pal = theme.palette();
            button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: match status {
                    Status::Active => pal.text,
                    Status::Hovered => ex.primary.weak.text,
                    Status::Disabled => ex.secondary.base.text,
                    Status::Pressed => ex.primary.base.text,
                },
                ..Default::default()
            }
        })
}

pub fn view_advanced<'a>(app: &'a App) -> Element<'a, Message> {
    let tools_by_category = Category::all().iter().map(|c| {
        (
            c,
            app.tools
                .iter()
                .enumerate()
                .filter(|(_, t)| t.category() == *c),
        )
    });

    let children = tools_by_category.map(|(c, tools)| {
        widget::column![
            text(c.name()).size(17).font(BOLD_DEFAULT),
            rule::horizontal(2), //
        ]
        .extend(tools.map(|(i, t)| {
            widget::row![
                container(space().width(6).height(Length::Fill)).style(|theme| container::Style {
                    background: Some(Background::Color(t.background(theme))),
                    ..Default::default()
                }),
                tool_small_button(t.as_ref(), i)
            ]
            .spacing(5)
            .height(20)
            .into()
        }))
        .spacing(5)
        .into()
    });

    let content = grid(children).fluid(200).spacing(20);

    let content = container(content).padding(PADDING);
    let view = scrollable(content);

    view.into()
}
