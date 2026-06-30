// hide console
#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

// TODO: add port scanning tool
// TODO: update showcase image in README.md

mod base;
mod message;
mod panic_handler;
mod tool;
mod ui;
mod window;

use iced::{
    Element, Length, Subscription, Task, clipboard, keyboard,
    widget::{self, *},
};

use base::ICON_FONT_BYTES;
use ipconfig::Adapter;

use crate::tool::Tool;
use crate::ui::Sidebar;
use crate::window::WindowHandler;

pub use message::Message;

/// Minimum AND default size for the window.
const WINDOW_MIN_SIZE: (f32, f32) = (870.0, 500.0);

fn main() {
    // catch panic and show a nice message box
    std::panic::set_hook(Box::new(panic_handler::handle_panic));

    iced::application(App::new, App::update, App::view)
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
        .run()
        .unwrap();
}

pub struct App {
    tools: Vec<Box<dyn Tool>>,
    selected: usize,
    theme: tool::settings::ThemeSetting,

    sidebar: Sidebar,
    window_handler: WindowHandler,
}

impl App {
    /// Returns the empty (default) state of the app, and returns a message that will properly load data.
    fn new() -> (Self, Task<Message>) {
        let tools = tool::all();

        let app = Self {
            selected: 0,
            theme: Default::default(),

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
        self.theme.into()
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
                    // keyboard::key::Named::Escape => Some(Message::GoHome),
                    _ => None,
                }
            }
            // track the cursor position (used for window dragging)
            Event::Mouse(me) => match me {
                MouseEvent::CursorMoved { position } => {
                    Some(Message::Window(window::Message::CursorMoved(position)))
                }
                _ => None,
            },
            // we need to have the window ID for later use
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
        match message {
            Message::Window(window::Message::CursorMoved(_)) | Message::Ignore => {}
            _ => debug!("=> MESSAGE: {message:#?}"),
        }

        match message {
            Message::Startup => {
                self.load_all_config();
                return self
                    .load_all_data()
                    .chain(Task::done(Message::SetTheme(self.theme))); // notify the settings tool we changed theme
            }
            Message::Window(window_message) => return self.window_handler.handle(window_message),

            Message::SetTheme(theme) => {
                self.theme = theme;
                // update settings as well
                return self.tools[self.selected].update(Message::SetTheme(theme));
            }

            Message::InitialDataLoaded(index, message) => {
                // We send it to the home tool as well, since it needs information from all the tools.
                // TODO: find a better solution instead of indexing.
                return self.tools[0]
                    .update(*message.clone())
                    .chain(self.tools[index].update(*message));
            }

            // sidebar
            Message::SidebarOptionSelected(index) => {
                self.selected = index;
                return self.tools[index].on_activate();
            }
            Message::CopyToClipboard(text) => {
                return clipboard::write(text);
            }

            // this is emitted by settings, but has to be handled globally because settings does not store `tools`
            Message::ResetAllSettings => {
                self.tools = tool::all();
                return self.load_all_data();
            }
            Message::OpenURL(url) => {
                let _ = open::that(url);
            }
            // Globally non-relevant Messages will be relegated to the `Tool`
            other => {
                return self.tools[self.selected].update(other);
            }
        }

        Task::none()
    }

    /// The main view of the application, contains:
    /// - Window decorations
    /// - Selected tool's view()
    /// - Sidebar
    fn view(&self) -> Element<'_, Message> {
        let content = self.tools[self.selected].view();

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

    /// Load all data in parallel
    fn load_all_data(&mut self) -> Task<Message> {
        Task::batch(self.tools.iter_mut().enumerate().map(|(i, t)| {
            // wrap the loaded data message in InitialDataLoaded so that it's dispatched to the tool
            t.load_data()
                .map(move |msg| crate::Message::InitialDataLoaded(i, Box::new(msg)))
        }))
    }

    /// Load config data into all the tools
    fn load_all_config(&mut self) {
        let path = Self::data_path();

        debug!("INFO: loading data from {path:?}");

        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_e) => {
                debug!("ERROR: failed to load save: {_e}");
                return;
            }
        };

        let map = match serde_json::from_slice(&bytes) {
            Ok(serde_json::Value::Object(m)) => m,
            Ok(_unexpected) => {
                debug!("ERROR: unexpected JSON value: {_unexpected}");
                return;
            }
            Err(_e) => {
                debug!("ERROR: failed to deserialize: {_e}");
                return;
            }
        };

        // global values
        if let Some(data) = map.get("theme") {
            match serde_json::from_value(data.clone()) {
                Ok(t) => self.theme = t,
                Err(_e) => {
                    debug!("ERROR: failed to load theme: {_e}");
                }
            }
        }

        // tool values
        for tool in &mut self.tools {
            if let Some(data) = map.get(tool.name()).cloned() {
                tool.load_config(data);
            }
        }
    }

    /// Collect state of all the `Tool`'s and saves it in a config file
    fn save_all_config(&self) {
        let mut data: serde_json::Map<String, serde_json::Value> = self
            .tools
            .iter()
            .filter_map(|t| t.save_config().map(|v| (t.name().to_owned(), v)))
            .collect();

        // store theme
        data.insert(
            "theme".to_owned(),
            serde_json::to_value(self.theme).unwrap(),
        );

        let data_dir = Self::data_dir();
        let path = Self::data_path();

        debug!("INFO: saving data to {path:?}");

        if let Err(_e) = std::fs::create_dir_all(&data_dir) {
            debug!("ERROR: failed to create {data_dir:?}: {_e}");
        }

        if let Ok(json) = serde_json::to_string_pretty(&data) {
            if let Err(_e) = std::fs::write(&path, json) {
                debug!("ERROR: failed to create {path:?}: {_e}");
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
