use crate::base::icon_font;
use iced::{
    Background, Color, Element, Length, Theme,
    widget::{self, button, container, rule, space, text},
};

use crate::tool::Tool;

type Message = crate::Message;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SidebarItem {
    Settings,
    Tool(usize),
}

impl SidebarItem {
    fn style_active(theme: &Theme, status: button::Status) -> button::Style {
        let ex = theme.extended_palette();

        let mut style = Self::style(theme, status);

        style.background = Some(Background::Color(ex.background.base.color));
        style.border = style.border.rounded(4);

        style
    }
    fn style(theme: &Theme, status: button::Status) -> button::Style {
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

    fn render(self, active: SidebarItem, tools: &[Box<dyn Tool>]) -> Element<'_, Message> {
        let is_active = active == self;

        let (name, icon) = match self {
            Self::Settings => ("Settings", icon_font::settings_gear()),
            Self::Tool(index) => (tools[index].name(), tools[index].icon()),
        };

        let text_size = 17;
        let button_contents = widget::row![
            icon.size(text_size),
            space().width(3),
            text(name).size(text_size)
        ];

        button(button_contents)
            .on_press(crate::Message::SidebarOption(self))
            .style(if is_active {
                Self::style_active
            } else {
                Self::style
            })
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug)]
pub struct Sidebar {
    items: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn from_tools(tools: &[Box<dyn Tool>]) -> Self {
        let items = tools
            .iter()
            .enumerate()
            .map(|(i, _t)| SidebarItem::Tool(i))
            .collect();

        Self { items }
    }

    pub fn view<'a>(
        &'a self,
        active: SidebarItem,
        tools: &'a [Box<dyn Tool>],
    ) -> Element<'a, Message> {
        // add header
        let mut col = widget::column![].height(Length::Fill).padding(10);

        // add items
        col = col.extend(
            self.items
                .iter()
                .map(|i| widget::column![i.render(active, tools), rule::horizontal(1)].into()),
        );

        // add footer
        col = col
            .push(space().height(Length::Fill))
            .push(rule::horizontal(2))
            .push(SidebarItem::Settings.render(active, tools));

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
