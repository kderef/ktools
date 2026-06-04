use iced::{
    Color, Element, Length, Padding, Task, Theme,
    widget::{self, button, container, space, text},
};

use crate::tool::{Category, Tool};

type Message = crate::Message;

#[derive(Debug)]
pub enum SidebarItem {
    Category {
        category: Category,
        expanded: bool,
        children: Vec<SidebarItem>,
        // on_click is not needed
    },
    Item {
        name: String,
        on_click: Message,
    },
}

#[derive(Debug, Clone)]
pub enum SidebarOption {
    Category(Category),
    Tool(Category, usize),
}

impl SidebarItem {
    fn style(theme: &Theme, status: button::Status) -> button::Style {
        use button::Status;

        let pal = theme.palette();
        let ex = theme.extended_palette();

        button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: match status {
                Status::Active => pal.text,
                Status::Pressed => pal.text,
                Status::Hovered => ex.secondary.base.color,
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

    fn render(&self) -> Element<'_, Message> {
        let view = match self {
            Self::Item { name, on_click } => button(text(name))
                .on_press(on_click.clone())
                .style(Self::style)
                .into(),
            Self::Category {
                category,
                expanded,
                children,
            } => {
                let on_click = Message::SidebarOption(SidebarOption::Category(*category));

                let view_self = button(widget::row![
                    category.icon(),
                    space().width(2),
                    category.name()
                ])
                .on_press(on_click)
                .style(Self::style);

                let mut col = widget::column![view_self];

                if *expanded {
                    for child in children {
                        col = col.push(widget::container(child.render()).padding(Padding {
                            top: 0.,
                            right: 0.,
                            bottom: 0.,
                            left: 16.,
                        }));
                    }
                }
                col.into()
            }
        };

        view
    }
}

pub fn sidebar_item<'a>(name: impl ToString, on_click: Message) -> SidebarItem {
    SidebarItem::Item {
        name: name.to_string(),
        on_click,
    }
}
pub fn sidebar_category(category: Category, children: Vec<SidebarItem>) -> SidebarItem {
    SidebarItem::Category {
        category,
        expanded: false,
        children,
    }
}

#[derive(Debug)]
pub struct Sidebar {
    items: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn from_tools(tools: &[Box<dyn Tool>]) -> Self {
        let items = Category::all()
            .iter()
            .map(|c| (c, tools.iter().filter(|t| t.category() == *c)))
            .map(|(c, ts)| {
                let children = ts
                    .enumerate()
                    .map(|(i, t)| {
                        sidebar_item(t.name(), Message::SidebarOption(SidebarOption::Tool(*c, i)))
                    })
                    .collect();

                let item = sidebar_category(*c, children);

                item
            })
            .collect();

        Self { items }
    }

    pub fn push(&mut self, item: SidebarItem) {
        self.items.push(item);
    }

    pub fn toggle_category(&mut self, category_toggled: Category) {
        for item in &mut self.items {
            if let SidebarItem::Category {
                category, expanded, ..
            } = item
            {
                if *category == category_toggled {
                    *expanded = !*expanded;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let col = widget::column(self.items.iter().map(SidebarItem::render)).height(Length::Fill);

        let view = container(col).style(|theme: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(
                theme.extended_palette().background.weaker.color,
            )),
            ..Default::default()
        });

        view.into()
    }
}
