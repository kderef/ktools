#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

// TODO: add port scanning tool
// TODO: revamp main home screen UI to have sections (network: [ext_ip, netinfo], ...)

mod base;
mod message;
mod tool;
mod window;

use iced::border::Radius;
use iced::{
    Background, Border, Color, Element, Length, Subscription, Task, clipboard, keyboard,
    widget::{self, *},
};

use base::ICON_FONT_BYTES;
use ipconfig::Adapter;

use crate::base::rgb8;
use crate::tool::Tool;
use crate::tool::settings::Settings;
use crate::window::WindowHandler;

pub use message::Message;

fn main() {
    iced::application(App::new, App::update, App::view)
        .window(iced::window::Settings {
            min_size: Some(iced::Size {
                width: 650.0,
                height: 500.0,
            }),
            // Avoid loading icon for faster debug build runtime
            icon: window::icon(),
            ..Default::default()
        })
        .title(App::title)
        .resizable(true)
        .window_size((650, 600))
        .decorations(false)
        .centered()
        .font(ICON_FONT_BYTES)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
        .unwrap();
}

/// Represents a selection of the user (home screen, settings screen, or a tool)
#[derive(Debug, Clone)]
enum Selection {
    Home,
    Settings,
    Tool(usize),
}

pub struct App {
    tools: Vec<Box<dyn Tool>>,
    selected: Selection,
    settings: Settings,

    window_handler: WindowHandler,
}

fn home_button<'a>(icon: Text<'a>, name: &'a str, bg: Color, index: usize) -> Button<'a, Message> {
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

impl App {
    /// Returns the empty (default) state of the app, and returns a message that will properly load data.
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            tools: tool::all(),
            selected: Selection::Home,
            settings: Settings::default(),
            window_handler: WindowHandler::new(),
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
            /* the rest */
            Message::GoHome => {
                self.selected = Selection::Home;
            }
            Message::GoToSettings => {
                self.selected = Selection::Settings;
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];

                if !tool.no_view() {
                    self.selected = Selection::Tool(index);
                }
                return tool.on_activate();
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
            // Globally non-relevant Messages will be relegated to the `Tool`
            other => match self.selected {
                Selection::Settings => return self.settings.update(other),
                Selection::Tool(index) => return self.tools[index].update(other),
                _ => {}
            },
        }

        Task::none()
    }

    /// Dynamic grid of squares representing tools.
    fn view(&self) -> Element<'_, Message> {
        let view = match self.selected {
            Selection::Settings => self.settings.view(),
            Selection::Tool(index) => self.tools[index].view(),
            Selection::Home => {
                // The grid of Tool's
                let children = self
                    .settings
                    .tool_order
                    .iter()
                    .filter_map(|name| self.tools.iter().position(|t| t.name() == name))
                    .map(|i| {
                        let t = &self.tools[i];
                        home_button(t.icon(), t.name(), t.background(&self.theme()), i).into()
                    });

                let grid = Grid::with_children(children).fluid(200).spacing(20);

                let content = Container::new(grid).padding(20);
                let view = Scrollable::new(content);

                widget::column![
                    view,
                    space().height(Length::Fill),
                    text("© Kian Heitkamp").size(11).color(rgb8(120, 120, 120))
                ]
                .into()
            }
        };

        let main_content = widget::column![window::decorations(self), view,]
            .height(Length::Fill)
            .width(Length::Fill);

        let view = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| container::Style {
                text_color: None,
                background: None,
                border: Border {
                    color: if cfg!(feature = "window-border-colored") {
                        theme.extended_palette().background.strongest.text
                    } else {
                        Color::TRANSPARENT
                    },
                    width: if cfg!(feature = "window-border-colored") {
                        1.0
                    } else {
                        0.0
                    },
                    radius: Radius::new(self.window_handler.window_border_radius),
                },
                ..Default::default()
            });

        let resize_areas = self.window_handler.resize_areas();

        stack![mouse_area(view).on_press(Message::Window(window::Message::Drag)),]
            .width(Length::Fill)
            .height(Length::Fill)
            .extend(resize_areas.into_iter())
            .into()
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
