use crate::Message;
use iced::{
    Alignment, Font, Length, Theme, futures,
    widget::{self, button, container, pick_list, row, rule, space, text, text_editor, text_input},
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::Stdio,
};
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

use super::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Ping {
    address: String,
    custom_address: bool,

    #[serde(skip)]
    output: text_editor::Content,

    #[serde(skip)]
    running: bool,
}

fn ping_stream(host: String) -> impl futures::Stream<Item = Message> {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    std::thread::spawn(move || {
        let mut child = match std::process::Command::new("ping")
            .arg(&host)
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

        let _ = child.wait();
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
            Message::PingAddressChanged(new) => {
                self.address = new;
            }
            Message::PingStart(addr) => {
                let addr = match addr {
                    Some(a) => {
                        self.custom_address = false;
                        a
                    }
                    None => self.address.clone(),
                };

                if addr.is_empty() || self.running {
                    return Task::none();
                }
                self.running = true;
                self.output = text_editor::Content::new();
                return Task::run(ping_stream(addr), |m| m);
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

        let ping_btn = custom_button(
            if self.running { "pinging..." } else { "ping" },
            Message::PingStart(None),
        )
        .width(Length::Fixed(90.0));

        let output = text_editor(&self.output)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .placeholder("ping output...")
            .on_action(Message::PingEditorAction); // make read-only by ignoring edits

        let ping_gateway_btn = custom_button(
            "Ping gateway",
            Message::PingStart(Some("8.8.8.8".to_owned())),
        );
        let ping_google_btn = custom_button(
            "Ping google",
            Message::PingStart(Some("google.com".to_owned())),
        );
        let ping_custom_btn = custom_button("Ping custom address", Message::PingToggleCustom);

        let mut content = widget::column![
            row![
                go_back_button(15),
                space().width(16),
                title_text(self).align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center),
            space().height(25),
            row![ping_gateway_btn, ping_google_btn, ping_custom_btn]
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
