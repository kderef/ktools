#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

#[cfg(not(debug_assertions))]
static ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

mod base;
mod tool;

use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Subscription;
use iced::Task;
use iced::clipboard;
use iced::keyboard;
use iced::widget;
use iced::widget::*;

use base::ICON_FONT_BYTES;

use crate::base::rgb8;
use crate::base::settings_button;
use crate::tool::Tool;

fn main() {
    iced::application(App::new, App::update, App::view)
        .window(iced::window::Settings {
            min_size: Some(iced::Size {
                width: 500.0,
                height: 400.0,
            }),
            // Avoid loading icon for faster debug build runtime
            #[cfg(not(debug_assertions))]
            icon: Some(
                iced::window::icon::from_file_data(ICON_BYTES, Some(::image::ImageFormat::Ico))
                    .unwrap(),
            ),
            ..Default::default()
        })
        .title(App::title)
        .resizable(true)
        .window_size((900, 600))
        .centered()
        .font(ICON_FONT_BYTES)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
        .unwrap();
}

/// Only message type used in the App.
/// It has a couple of generic messages such as `GoHome`
/// and a couple of `Tool`-specific messages such as `ExternalIpFetched()`
#[derive(Debug, Clone)]
pub enum Message {
    /// Runs once when the window is opened
    Startup,

    /* Home page messages */
    /// Go to index
    ChooseTool(usize),

    /// Reset `current_tool` to `None`
    GoHome,

    /* Generic messages */
    Refresh,
    CategorySelected(usize),
    TabSelected(usize),
    CopyToClipboard(String),
    TopTabSelected(usize),

    /* messages for passgen */
    PasswordGenerator(tool::passgen::Message),

    /* messages for ext_ip */
    ExternalIpFetched(Result<tool::ext_ip::Object, String>),

    /* messages for sys_info */
    SystemInfoFetched(&'static str, Result<tool::sys_info::SystemValue, String>),
}

pub struct App {
    tools: Vec<Box<dyn Tool>>,
    selected_tool: Option<usize>,
}

fn home_button<'a>(
    icon: Text<'a>,
    name: &'a str,
    bg: Color,
    text_color: Color,
    index: usize,
) -> Button<'a, Message> {
    let icon = icon.size(28).color(text_color);
    button(
        container(
            iced::widget::column![icon, text(name).size(16).color(text_color),]
                .align_x(iced::Alignment::Center)
                .spacing(8),
        )
        .center(Length::Fill),
    )
    .width(Length::Fixed(160.0))
    .height(Length::Fixed(80.0))
    .on_press(Message::ChooseTool(index))
    .style(move |_theme: &Theme, status| {
        let alpha = match status {
            button::Status::Hovered => 0.82,
            button::Status::Pressed => 0.65,
            _ => 1.0,
        };
        let tinted = Color { a: alpha, ..bg };
        button::Style {
            snap: false,
            background: Some(Background::Color(tinted)),
            text_color,
            border: Border {
                color: Color::from_rgba(1., 1., 1., 0.3),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 6.0,
            },
        }
    })
}

impl App {
    /// Returns the empty (default) state of the app, and returns a message that will properly load data.
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            tools: tool::all(),
            selected_tool: None,
        };

        (app, Task::done(Message::Startup))
    }

    fn title(&self) -> String {
        format!("KTools v{}", env!("CARGO_PKG_VERSION"))
    }

    fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed {
                modified_key: keyboard::Key::Named(modified_key),
                repeat: false,
                ..
            } = event
            else {
                return None;
            };

            match modified_key {
                keyboard::key::Named::Escape => Some(Message::GoHome),
                _ => None,
            }
        })
    }

    /// NOTE: only globally relevant messages such as `CopyToClipboard` will be handled here.
    /// The rest will be relegated to the currently selected `Tool`
    fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
        println!("=> MESSAGE: {message:#?}");

        match message {
            Message::Startup => {
                self.load_all();
            }
            Message::GoHome => {
                self.selected_tool = None;
            }
            Message::ChooseTool(index) => {
                let tool = &mut self.tools[index];

                if !tool.no_view() {
                    self.selected_tool = Some(index);
                }
                return tool.on_activate();
            }
            Message::CopyToClipboard(text) => {
                return clipboard::write(text);
            }
            // Globally non-relevant Messages will be relegated to the `Tool`
            other => {
                if let Some(index) = self.selected_tool {
                    return self.tools[index].update(other);
                }
            }
        }

        Task::none()
    }

    /// Dynamic grid of squares representing tools.
    fn view(&self) -> Element<'_, Message> {
        match self.selected_tool {
            Some(index) => self.tools[index].view(),
            None => {
                // top bar
                let top = row![
                    settings_button(&*self.tools[0]),
                    space().width(Length::Fill),
                    text("KTools")
                        .size(35)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .center(),
                    space().width(Length::Fill),
                ]
                .width(Length::Fill);

                // the grid
                let children =
                    self.tools
                        .iter()
                        .filter(|t| !t.hidden())
                        .enumerate()
                        .map(|(i, t)| {
                            home_button(t.icon(), t.name(), t.background(), t.text_color(), i)
                                .into()
                        });

                let grid = Grid::with_children(children).fluid(200).spacing(20);

                let content = Container::new(grid).padding(20);
                let view = Scrollable::new(content);

                widget::column![
                    top,
                    space().height(2),
                    row![space().width(20), rule::horizontal(2), space().width(20)],
                    view,
                    space().height(Length::Fill),
                    text("© Kian Heitkamp").size(11).color(rgb8(120, 120, 120))
                ]
                .into()
            }
        }
    }

    /// Load saved data into all the tools
    fn load_all(&mut self) {
        let path = Self::data_path();
        let Ok(bytes) = std::fs::read(&path) else {
            return;
        };
        let Ok(serde_json::Value::Object(map)) = serde_json::from_slice(&bytes) else {
            return;
        };

        #[cfg(debug_assertions)]
        println!("INFO: loading data from {:?}", path);

        for tool in &mut self.tools {
            if let Some(data) = map.get(tool.name()).cloned() {
                tool.load(data);
            }
        }
    }

    /// Collect state of all the `Tool`'s and saves it in a config file
    fn save_all(&self) {
        let data: serde_json::Map<String, serde_json::Value> = self
            .tools
            .iter()
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
