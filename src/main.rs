#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

// TODO: add port scanning tool
// TODO: revamp main home screen UI to have sections (network: [ext_ip, netinfo], ...)

mod base;
mod homescreen;
mod message;
mod tool;
mod ui;
mod window;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use iced::{
    Element, Length, Subscription, Task, clipboard, keyboard,
    widget::{self, *},
};

use base::ICON_FONT_BYTES;
use ipconfig::Adapter;

use crate::homescreen::HomescreenStyle;
use crate::tool::Tool;
use crate::tool::settings::Settings;
use crate::ui::{Sidebar, SidebarItem};
use crate::window::WindowHandler;

pub use message::Message;

fn main() {
    iced::application(App::new, App::update, App::view)
        .window(iced::window::Settings {
            min_size: Some(iced::Size {
                width: 850.0,
                height: 500.0,
            }),
            icon: window::icon(),
            ..Default::default()
        })
        .title(App::title)
        .resizable(true)
        .window_size((850, 600))
        .decorations(false)
        .centered()
        .font(ICON_FONT_BYTES)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
        .unwrap();
}

pub struct App {
    tools: Vec<Box<dyn Tool>>,
    selected: SidebarItem,
    settings: Settings,

    // searching
    search: String,
    search_matches: Vec<usize>,

    sidebar: Sidebar,
    window_handler: WindowHandler,
}

impl App {
    /// Returns the empty (default) state of the app, and returns a message that will properly load data.
    fn new() -> (Self, Task<Message>) {
        let tools = tool::all();
        let app = Self {
            selected: SidebarItem::Home,
            settings: Settings::default(),
            search: String::new(),
            search_matches: tools.iter().enumerate().map(|(i, _)| i).collect(),
            window_handler: WindowHandler::new(),
            sidebar: Sidebar::from_tools(&tools),
            tools,
        };

        (app, Task::done(Message::Startup))
    }

    fn title(&self) -> String {
        format!("KTools v{}", env!("CARGO_PKG_VERSION"))
    }

    fn theme(&self) -> iced::Theme {
        self.settings.theme.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced::Event;
        use iced::mouse::Event as MouseEvent;
        use iced::window::Event as WindowEvent;

        iced::event::listen_with(|event, _status, id| match event {
            Event::Keyboard(e) => {
                let keyboard::Event::KeyPressed {
                    modified_key: keyboard::Key::Named(modified_key),
                    repeat: false,
                    ..
                } = e
                else {
                    return None;
                };

                match modified_key {
                    keyboard::key::Named::Escape => Some(Message::GoHome),
                    keyboard::key::Named::F5 => Some(Message::Refresh),
                    _ => None,
                }
            }
            Event::Mouse(me) => match me {
                MouseEvent::CursorMoved { position } => {
                    Some(Message::Window(window::Message::CursorMoved(position)))
                }
                _ => None,
            },
            Event::Window(we) => match we {
                WindowEvent::Opened { position: _, size } => {
                    Some(Message::Window(window::Message::Opened { id, size }))
                }
                _ => None,
            },
            _ => None,
        })
    }

    /// NOTE: only globally relevant messages such as `CopyToClipboard` will be handled here.
    /// The rest will be relegated to the currently selected `Tool`
    fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
        if !matches!(message, Message::Window(window::Message::CursorMoved(_))) {
            println!("=> MESSAGE: {message:#?}");
        }

        match message {
            Message::Startup => {
                self.load_all();
            }
            Message::Window(window_message) => return self.window_handler.handle(window_message),
            Message::GoHome => {
                self.selected = SidebarItem::Home;
            }
            Message::GoToSettings => {
                self.selected = SidebarItem::Settings;
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];

                if !tool.no_view() {
                    self.selected = SidebarItem::Tool(index);
                }
                return tool.on_activate();
            }

            // sidebar
            Message::SidebarOption(opt) => {
                match opt {
                    SidebarItem::Home => {}
                    SidebarItem::Settings => {}
                    SidebarItem::Tool(index) => {
                        let tool = &mut self.tools[index];

                        if !tool.no_view() {
                            self.selected = SidebarItem::Tool(index);
                        }
                        return tool.on_activate();
                    }
                }
                self.selected = opt;
            }
            Message::CopyToClipboard(text) => {
                return clipboard::write(text);
            }
            Message::ResetAllSettings => {
                self.settings = Settings::default();
                self.tools = tool::all();
            }
            Message::OpenURL(url) => {
                let _ = open::that(url);
            }
            Message::Search(query) => {
                let query_lower = query.to_lowercase();

                let matcher = SkimMatcherV2::default();

                let tool_indices = self.tools.iter().enumerate().map(|(i, t)| (i, t.name()));

                let mut matches: Vec<(usize, i64)> = tool_indices
                    .filter_map(|(i, tn)| {
                        matcher
                            .fuzzy_match(&tn.to_lowercase(), &query_lower)
                            .map(|score| (i, score))
                    })
                    .collect();

                matches.sort_by(|a, b| b.1.cmp(&a.1));

                self.search = query;
                self.search_matches = matches.iter().map(|(i, _)| *i).collect();
            }
            Message::SetHomescreenStyle(style) => {
                self.settings.homescreen_style = style;
            }
            // Globally non-relevant Messages will be relegated to the `Tool`
            other => match self.selected {
                SidebarItem::Settings => return self.settings.update(other),
                SidebarItem::Tool(index) => return self.tools[index].update(other),
                _ => {}
            },
        }

        Task::none()
    }

    /// Dynamic grid of squares representing tools.

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self.selected {
            SidebarItem::Settings => self.settings.view(),
            SidebarItem::Tool(index) => self.tools[index].view(),
            SidebarItem::Home => match self.settings.homescreen_style {
                HomescreenStyle::Simple => homescreen::view_simple(self),
                HomescreenStyle::List => homescreen::view_advanced(self),
            },
        };

        let decorations = window::decorations(self, false);
        let titlebar_text = window::titlebar_text(self).width(Length::Fill);

        // Decorations + content stacked in the right column only
        let right = widget::column![decorations, content,]
            .height(Length::Fill)
            .width(Length::Fill);

        // Sidebar spans full height — its background paints over the titlebar area
        let main_content = widget::row![self.sidebar.view(self.selected, &self.tools), right,]
            .height(Length::Fill)
            .width(Length::Fill);

        // to make sure the window title is centered
        let main_content = stack![main_content, titlebar_text,];

        let view = self.window_handler.container(main_content);
        self.window_handler.wrap(view)
    }
    /// Load saved data into all the tools
    fn load_all(&mut self) {
        let path = Self::data_path();

        #[cfg(debug_assertions)]
        println!("INFO: loading data from {path:?}");

        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_e) => {
                #[cfg(debug_assertions)]
                eprintln!("ERROR: failed to load save: {_e}");
                return;
            }
        };

        let map = match serde_json::from_slice(&bytes) {
            Ok(serde_json::Value::Object(m)) => m,
            Ok(_unexpected) => {
                #[cfg(debug_assertions)]
                eprintln!("ERROR: unexpected JSON value: {_unexpected}");
                return;
            }
            Err(_e) => {
                #[cfg(debug_assertions)]
                eprintln!("ERROR: failed to deserialize: {_e}");
                return;
            }
        };

        for tool in &mut self.tools {
            if let Some(data) = map.get(tool.name()).cloned() {
                tool.load(data);
            }
        }

        if let Some(data) = map.get(self.settings.name()).cloned() {
            self.settings.load(data);
        }
    }

    /// Collect state of all the `Tool`'s and saves it in a config file
    fn save_all(&self) {
        let data: serde_json::Map<String, serde_json::Value> = self
            .tools
            .iter()
            .map(|t| t.as_ref()) // unbox
            .chain([&self.settings as &dyn Tool]) // add settings
            .filter_map(|t| t.save().map(|v| (t.name().to_owned(), v)))
            .collect();

        let data_dir = Self::data_dir();
        let path = Self::data_path();

        #[cfg(debug_assertions)]
        println!("INFO: saving data to {path:?}");

        if let Err(e) = std::fs::create_dir_all(&data_dir)
            && cfg!(debug_assertions)
        {
            eprintln!("ERROR: failed to create {data_dir:?}: {e}");
        }

        if let Ok(json) = serde_json::to_string_pretty(&data) {
            if let Err(e) = std::fs::write(&path, json)
                && cfg!(debug_assertions)
            {
                eprintln!("ERROR: failed to create {path:?}: {e}");
            }
        }
    }

    /// Directory path for the app's config folder
    fn data_dir() -> std::path::PathBuf {
        dirs::data_local_dir().unwrap_or(".".into()).join("ktools")
    }
    /// Path for the userdata file in the `data_dir` folder.
    fn data_path() -> std::path::PathBuf {
        Self::data_dir().join("userdata.json")
    }
}

// IMPORTANT: we save userdata on exit (after window closes)
impl Drop for App {
    fn drop(&mut self) {
        self.save_all();
    }
}
