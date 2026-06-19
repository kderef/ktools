#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

// TODO: add port scanning tool
// TODO: add homescreen to tool/home.rs

// TODO(fix): Sometimes InitialDataLoaded message is not sent on startup.

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

use crate::tool::Tool;
use crate::tool::settings::Settings;
use crate::ui::{Sidebar, SidebarItem};
use crate::window::WindowHandler;

pub use message::Message;

/// Minimum AND default size for the window.
const WINDOW_MIN_SIZE: (f32, f32) = (870.0, 500.0);

fn main() {
    let app_result = iced::application(App::new, App::update, App::view)
        .window(iced::window::Settings {
            min_size: Some(WINDOW_MIN_SIZE.into()),
            icon: window::icon(),
            ..Default::default()
        })
        .title(App::title)
        .resizable(true)
        .window_size(WINDOW_MIN_SIZE)
        .decorations(false)
        .centered()
        .font(ICON_FONT_BYTES)
        .theme(App::theme)
        .subscription(App::subscription)
        .run();

    if let Err(e) = app_result {
        #[cfg(debug_assertions)]
        eprintln!("FATAL APP ERROR: {e:?}");

        ui::messagebox_err("KTools fatal error", &e.to_string());
        std::process::exit(1);
    }
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
            selected: SidebarItem::Tool(0),
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
                self.load_all_config();
                return self.load_all_data();
            }
            Message::Window(window_message) => return self.window_handler.handle(window_message),
            Message::GoHome => {
                self.selected = SidebarItem::Tool(0);
                return self.tools[0].on_activate();
            }
            Message::GoToSettings => {
                self.selected = SidebarItem::Settings;
                return self.settings.on_activate();
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];

                self.selected = SidebarItem::Tool(index);
                return tool.on_activate();
            }

            Message::InitialDataLoaded(index, message) => {
                return self.all_tools_mut().nth(index).unwrap().update(*message);
            }

            // sidebar
            Message::SidebarOption(opt) => match opt {
                SidebarItem::Settings => {
                    self.selected = opt;
                    return self.settings.on_activate();
                }
                SidebarItem::Tool(index) => {
                    let tool = &mut self.tools[index];

                    self.selected = SidebarItem::Tool(index);
                    return tool.on_activate();
                }
            },
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
            // Globally non-relevant Messages will be relegated to the `Tool`
            other => match self.selected {
                SidebarItem::Settings => return self.settings.update(other),
                SidebarItem::Tool(index) => return self.tools[index].update(other),
            },
        }

        Task::none()
    }

    /// The main view of the application, contains:
    /// - Window decorations
    /// - Selected tool's view()
    /// - Sidebar
    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self.selected {
            SidebarItem::Settings => self.settings.view(),
            SidebarItem::Tool(index) => self.tools[index].view(),
        };

        let decorations = self.window_handler.decorations();
        let titlebar_text = self
            .window_handler
            .titlebar_text(self.selected, &self.tools)
            .width(Length::Fill);

        // Decorations + content stacked in the right column only
        let right = widget::column![decorations, content]
            .height(Length::Fill)
            .width(Length::Fill);

        // Sidebar of the app, takes up FULL height, including the top where the window decorations are.
        let main_content = widget::row![self.sidebar.view(self.selected, &self.tools), right]
            .height(Length::Fill)
            .width(Length::Fill);

        // to make sure the window title is centered
        let main_content = stack![main_content, titlebar_text,];

        let view = self.window_handler.container(main_content);
        self.window_handler.wrap(view)
    }

    /// All the tools including edge cases such as `Settings` and `Home`
    fn all_tools_mut(&mut self) -> impl Iterator<Item = &mut dyn Tool> {
        self.tools
            .iter_mut()
            .map(|t| t.as_mut() as &mut dyn Tool)
            .chain([&mut self.settings as &mut dyn Tool])
    }
    /// All the tools including edge cases such as `Settings` and `Home`
    fn all_tools(&self) -> impl Iterator<Item = &dyn Tool> {
        self.tools
            .iter()
            .map(Box::as_ref)
            .chain([&self.settings as &dyn Tool])
    }

    fn load_all_data(&mut self) -> Task<Message> {
        Task::batch(self.all_tools_mut().enumerate().map(|(i, t)| {
            t.load_data()
                .map(move |msg| crate::Message::InitialDataLoaded(i, Box::new(msg)))
        }))
    }

    /// Load config data into all the tools
    fn load_all_config(&mut self) {
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

        for tool in self.all_tools_mut() {
            if let Some(data) = map.get(tool.name()).cloned() {
                tool.load_config(data);
            }
        }
    }

    /// Collect state of all the `Tool`'s and saves it in a config file
    fn save_all_config(&self) {
        let data: serde_json::Map<String, serde_json::Value> = self
            .all_tools()
            .filter_map(|t| t.save_config().map(|v| (t.name().to_owned(), v)))
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
        self.save_all_config();
    }
}
