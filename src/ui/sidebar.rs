use iced::{
    Element, Length, Padding, Task, Theme,
    widget::{self, button, text},
};

use crate::tool::{Category, Tool};

type Message = crate::Message;

pub struct SidebarItem {
    pub name: String,
    pub expanded: bool,
    pub children: Vec<Self>,
    pub on_click: Message,
}

#[derive(Debug, Clone)]
pub enum SidebarOption {
    Category(Category),
    Tool(Category, usize),
}

impl SidebarItem {
    fn style(theme: &Theme, status: button::Status) -> button::Style {
        let pal = theme.palette();
        let ex = theme.extended_palette();
        todo!()
    }

    fn render(&self) -> Element<'_, Message> {
        let mut col = widget::column![
            button(text(&self.name))
                .on_press(self.on_click.clone())
                .style(Self::style)
        ];

        if self.expanded {
            for child in &self.children {
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
}

pub fn sidebar_item<'a>(name: impl ToString, on_click: Message) -> SidebarItem {
    SidebarItem {
        name: name.to_string(),
        expanded: false,
        children: vec![],
        on_click,
    }
}

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
                let mut item = sidebar_item(
                    c.name(),
                    Message::SidebarOption(SidebarOption::Category(*c)),
                );
                item.children = ts
                    .enumerate()
                    .map(|(i, t)| {
                        sidebar_item(t.name(), Message::SidebarOption(SidebarOption::Tool(*c, i)))
                    })
                    .collect();

                item
            })
            .collect();

        Self { items }
    }

    pub fn push(&mut self, item: SidebarItem) {
        self.items.push(item);
    }
    pub fn view(&self) -> Element<'_, Message> {
        widget::column(self.items.iter().map(SidebarItem::render))
            .height(Length::Fill)
            .into()
    }
}
