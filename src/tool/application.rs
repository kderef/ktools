use std::os::windows::process::CommandExt;

use iced::{
    Background, Border, Length,
    widget::{Button, button, container, grid, scrollable, text},
};

use crate::Message;

use super::*;

enum AppBackground {
    Black,
    Orange,
}

pub struct App {
    name: &'static str,
    icon: fn() -> Text<'static>,
    cmd: &'static [&'static str],
    create_console: bool,
    elevate: bool,
    background: AppBackground,
}
impl App {
    const fn create_console(mut self) -> Self {
        self.create_console = true;
        self
    }
    const fn elevate(mut self) -> Self {
        self.elevate = true;
        self
    }
    const fn new(
        name: &'static str,
        icon: fn() -> Text<'static>,
        cmd: &'static [&'static str],
        bg: AppBackground,
    ) -> Self {
        Self {
            name,
            icon,
            cmd,
            create_console: false,
            elevate: false,
            background: bg,
        }
    }
}

pub struct Applications {
    apps: Vec<App>,
}

impl Default for Applications {
    fn default() -> Self {
        Self {
            apps: vec![
                App::new(
                    "CMD",
                    icon_font::terminal_cmd,
                    &["cmd.exe"],
                    AppBackground::Black,
                )
                .create_console(), //
                App::new(
                    "CMD (admin)",
                    icon_font::terminal_cmd,
                    &["cmd.exe"],
                    AppBackground::Orange,
                )
                .create_console()
                .elevate(),
            ],
        }
    }
}

pub fn app_button<'a>(app: &'a App) -> Button<'a, Message> {
    let icon = (app.icon)().size(28);
    let name = app.name;
    let message = Message::ApplicationOpen {
        cmd: app.cmd,
        create_new_console: app.create_console,
        elevate: app.elevate,
    };

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
    .on_press(message)
    .style(move |theme: &Theme, status| {
        let alpha = match status {
            button::Status::Hovered => 0.82,
            button::Status::Pressed => 0.65,
            _ => 1.0,
        };
        let bg = match app.background {
            AppBackground::Black => Color::BLACK,
            AppBackground::Orange => theme.extended_palette().warning.weak.color,
        };
        let tinted = Color { a: alpha, ..bg };
        button::Style {
            snap: false,
            background: Some(Background::Color(tinted)),
            text_color: rgb8(255, 255, 255),
            border: Border {
                color: theme.extended_palette().secondary.base.color,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        }
    })
}

impl Tool for Applications {
    fn name(&self) -> &'static str {
        "Applications"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::terminal_cmd()
    }
    fn background(&self, _theme: &Theme) -> Color {
        Color::default()
    }

    fn save(&self) -> Option<serde_json::Value> {
        None
    }
    fn load(&mut self, _data: serde_json::Value) {}
    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::none()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        const CREATE_NEW_CONSOLE: u32 = 0x00000010;

        match message {
            Message::ApplicationOpen {
                cmd,
                create_new_console,
                elevate,
            } => {
                let program = cmd[0];
                let args = &cmd[1..];

                if elevate {
                    run_elevated(&cmd.join(" "));
                } else {
                    let mut process = std::process::Command::new(program);

                    if create_new_console {
                        process.creation_flags(CREATE_NEW_CONSOLE);
                    }

                    let _result = process.args(args).spawn();

                    #[cfg(debug_assertions)]
                    println!("$ {program} {args:?} -> {_result:?}");
                }
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        const PADDING: u16 = 20;

        // The grid of Tool's
        let children = self.apps.iter().map(app_button).map(Into::into);

        let grid = grid(children).fluid(140).spacing(20);

        let content = container(grid).padding(PADDING);
        let view = scrollable(content);

        view.into()
    }
}

#[cfg(target_os = "windows")]
fn run_elevated(cmd: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::core::PCWSTR;

    fn wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    unsafe {
        ShellExecuteW(
            Some(HWND(std::ptr::null_mut())),
            PCWSTR(wide("runas").as_ptr()),
            PCWSTR(wide(cmd).as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
        );
    }
}
