use crate::tool::SidebarPosition;
use iced::{
    Background, Color, Element, Length, Theme,
    widget::{self, button, container, rule, space, text},
};

use crate::tool::Tool;

type Message = crate::Message;

fn sidebar_item_style_active(theme: &Theme, status: button::Status) -> button::Style {
    let ex = theme.extended_palette();

    let mut style = sidebar_item_style(theme, status);

    style.background = Some(Background::Color(ex.background.base.color));
    style.border = style.border.rounded(4);

    style
}
fn sidebar_item_style(theme: &Theme, status: button::Status) -> button::Style {
    use button::Status;

    let pal = theme.palette();
    let ex = theme.extended_palette();

    button::Style {
        background: match status {
            Status::Hovered => Some(Background::Color(ex.background.weakest.color)),
            _ => Some(Background::Color(Color::TRANSPARENT)),
        },
        text_color: match status {
            Status::Active | Status::Hovered => pal.text,
            Status::Pressed => pal.text,
            Status::Disabled => pal.text,
        },
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(0),
        },
        shadow: Default::default(),
        snap: Default::default(),
    }
}

fn sidebar_item_render(
    item: usize,
    active: usize,
    tools: &[Box<dyn Tool>],
) -> Element<'_, Message> {
    let is_active = active == item;

    let (name, icon) = (tools[item].name(), tools[item].icon());

    let text_size = 17;
    let button_contents = widget::row![
        icon.size(text_size),
        space().width(4),
        text(name).size(text_size)
    ];

    button(button_contents)
        .on_press(crate::Message::SidebarOptionSelected(item))
        .style(if is_active {
            sidebar_item_style_active
        } else {
            sidebar_item_style
        })
        .width(Length::Fill)
        .into()
}

#[derive(Debug)]
pub struct Sidebar {
    items: Vec<(usize, SidebarPosition)>,
}

impl Sidebar {
    pub fn from_tools(tools: &[Box<dyn Tool>]) -> Self {
        let items = tools
            .iter()
            .enumerate()
            .map(|(i, t)| (i, t.sidebar_position()))
            .collect();

        Self { items }
    }

    pub fn view<'a>(&'a self, active: usize, tools: &'a [Box<dyn Tool>]) -> Element<'a, Message> {
        // add header
        let mut col = widget::column(
            self.items
                .iter()
                .filter(|(_, pos)| *pos == SidebarPosition::Top)
                .map(|(i, _)| {
                    widget::column![sidebar_item_render(*i, active, tools), rule::horizontal(1)]
                        .into()
                }),
        )
        .push(rule::horizontal(2))
        .push(space().height(Length::Fixed(10.0)))
        .height(Length::Fill)
        .padding(10);

        // add middle
        col = col.extend(
            self.items
                .iter()
                .filter(|(_, pos)| *pos == SidebarPosition::Middle)
                .map(|(i, _)| {
                    widget::column![sidebar_item_render(*i, active, tools), rule::horizontal(1)]
                        .into()
                }),
        );

        // add space before bottom
        col = col
            .push(space().height(Length::Fill))
            .push(rule::horizontal(2));

        // add bottom items
        col = col.extend(
            self.items
                .iter()
                .filter(|(_, pos)| *pos == SidebarPosition::Bottom)
                .map(|(i, _)| {
                    widget::column![sidebar_item_render(*i, active, tools), rule::horizontal(1)]
                        .into()
                }),
        );

        let row = widget::row![col, rule::vertical(2)];

        let view = container(row)
            .style(|theme: &Theme| container::Style {
                text_color: None,
                background: Some(iced::Background::Color(
                    theme.extended_palette().background.weaker.color,
                )),
                ..Default::default()
            })
            .width(220);

        view.into()
    }
}
