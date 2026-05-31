use crate::Message;
use iced::{
    Alignment, Font, Length, Theme, futures,
    widget::{self, button, container, row, space, text, text_editor, text_input},
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    net::IpAddr,
    os::windows::process::CommandExt,
    process::Stdio,
    sync::{Arc, Mutex, MutexGuard},
};
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

use super::*;

type ChildHandle = Arc<Mutex<Option<std::process::Child>>>;

fn lock(handle: &ChildHandle) -> MutexGuard<'_, Option<std::process::Child>> {
    handle.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Default, Serialize, Deserialize)]
pub struct Ping {
    address: String,
    custom_address: bool,

    #[serde(skip)]
    default_gateway: Option<Result<String, String>>,

    #[serde(skip)]
    output: text_editor::Content,

    /// Whether a ping process is currently running
    #[serde(skip)]
    running: bool,

    #[serde(skip)]
    child: Option<ChildHandle>,
}

fn get_default_gateway() -> Result<sys_info::SystemValue, String> {
    let adapters = ipconfig::get_adapters().map_err(|e| e.to_string())?;

    for adapter in adapters {
        // Skip disconnected adapters
        if adapter.oper_status() != ipconfig::OperStatus::IfOperStatusUp {
            continue;
        }

        for gateway in adapter.gateways() {
            if let IpAddr::V4(ip) = gateway {
                return Ok(sys_info::SystemValue::Text(ip.to_string()));
            }
        }
    }

    Err("No default gateway found".into())
}

fn ping_stream(host: String, child_handle: ChildHandle) -> impl futures::Stream<Item = Message> {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    std::thread::spawn(move || {
        let parts = host.trim().split_whitespace().collect::<Vec<&str>>();

        let host_addr = parts[0];
        let extra_args = &parts[1..];

        let mut child = match std::process::Command::new("ping")
            .arg(host_addr)
            .args(extra_args)
            .stdout(Stdio::piped())
            .creation_flags(CREATE_NO_WINDOW.0)
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.unbounded_send(Message::PingOutput(format!("Failed to start: {e}")));
                let _ = tx.unbounded_send(Message::PingDone);
                return;
            }
        };

        let stdout = child.stdout.take().unwrap();
        *lock(&child_handle) = Some(child);

        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if tx.unbounded_send(Message::PingOutput(line)).is_err() {
                        break; // receiver dropped, user navigated away
                    }
                }
                Err(_) => break,
            }
        }

        if let Some(c) = lock(&child_handle).as_mut() {
            let _ = c.wait();
        }

        *lock(&child_handle) = None;

        let _ = tx.unbounded_send(Message::PingDone);
    });

    rx
}
impl Tool for Ping {
    fn name(&self) -> &str {
        "Ping"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::debug_disconnect()
    }
    fn background(&self, _theme: &Theme) -> Color {
        rgb8(100, 100, 100)
    }
    fn save(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
    fn load(&mut self, data: serde_json::Value) {
        if let Ok(data) = serde_json::from_value(data) {
            *self = data;
        }
    }
    fn on_activate(&mut self) -> Task<Message> {
        Task::none()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SystemInfoFetched(_, result) => {
                self.default_gateway = Some(result.map(|r| r.to_string()));
            }

            Message::PingAddressChanged(new) => {
                self.address = new;
            }
            Message::PingDefaultGateway => match &self.default_gateway {
                Some(Ok(addr)) => return Task::done(Message::PingStart(Some(addr.clone()))),
                _ => {
                    self.output = Default::default(); // clear output before showing status messages
                    return Task::chain(
                        Task::done(Message::PingOutput("Loading default gateway...".to_owned())),
                        Task::perform(async { get_default_gateway() }, |r| match r {
                            Ok(addr) => Message::PingStart(Some(addr.to_string())),
                            Err(e) => Message::PingOutput(format!("ERROR: {e}")),
                        }),
                    );
                }
            },
            Message::PingCancel => {
                self.running = false;

                // If a process exists, kill it.
                if let Some(handle) = &self.child {
                    if let Some(child) = lock(handle).as_mut() {
                        let _ = child.kill();
                    }
                }

                self.child = None;

                return Task::done(Message::PingOutput("Ping canceled".to_owned()));
            }
            Message::PingStart(addr) => {
                let addr = match addr {
                    Some(a) => {
                        self.custom_address = false;
                        a
                    }
                    None => self.address.clone(),
                };

                if addr.trim().is_empty() || self.running {
                    return Task::none();
                }

                // Begin the process
                self.running = true;
                self.output = text_editor::Content::new();

                // create a new child instance
                let handle = Arc::new(Mutex::new(None));
                self.child = Some(handle.clone());

                return Task::run(ping_stream(addr, handle), |m| m);
            }
            Message::PingToggleCustom => {
                self.custom_address ^= true;
            }
            Message::PingOutput(line) => {
                let mut current = self.output.text();
                if !current.is_empty() {
                    current.push('\n');
                }
                current.push_str(&line);
                self.output = text_editor::Content::with_text(&current);
            }
            Message::PingEditorAction(action) => {
                if !action.is_edit() {
                    self.output.perform(action);
                }
            }
            Message::PingDone => {
                self.running = false;
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let input = text_input("Address to ping...", &self.address)
            .on_input(Message::PingAddressChanged)
            .on_submit(Message::PingStart(None));

        let custom_button = |txt: &'static str, message| {
            button(text(txt).size(15).center()).on_press_maybe((!self.running).then_some(message))
        };

        let ping_btn = if self.running {
            custom_button("Stop Ping", Message::PingCancel)
                .style(button::danger)
                .on_press(Message::PingCancel)
        } else {
            custom_button("Ping", Message::PingStart(None)).on_press(Message::PingStart(None))
        }
        .width(Length::Fixed(90.0));

        let output = text_editor(&self.output)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .placeholder("ping output...")
            .on_action(Message::PingEditorAction); // make read-only by ignoring edits

        let ping_gateway_btn = custom_button("Ping gateway", Message::PingDefaultGateway);
        let ping_google_btn = custom_button(
            "Ping google.com",
            Message::PingStart(Some("google.com".to_owned())),
        );
        let ping_google_dns_btn = custom_button(
            "Ping google DNS (8.8.8.8)",
            Message::PingStart(Some("8.8.8.8".to_owned())),
        );
        let ping_custom_btn = custom_button("Ping custom address", Message::PingToggleCustom);

        let mut content = widget::column![
            row![
                ping_gateway_btn,
                ping_google_btn,
                ping_google_dns_btn,
                ping_custom_btn
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .height(Length::Fill);

        if self.custom_address {
            content = content
                .push(space().height(8))
                .push(row![input, ping_btn].spacing(8).align_y(Alignment::Center));
        }

        content = content
            .push(space().height(8)) //
            .push(output);

        let container = container(content).padding(12).height(Length::Fill);

        container.into()
    }
}
