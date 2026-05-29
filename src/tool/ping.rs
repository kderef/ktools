use crate::Message;
use iced::{
    Alignment, Font, Length, Theme, futures,
    widget::{self, button, container, row, space, text, text_editor, text_input},
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
};

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Ping {
    address: String,
    #[serde(skip)]
    output: text_editor::Content,
    #[serde(skip)]
    running: bool,
}

impl Default for Ping {
    fn default() -> Self {
        Self {
            address: "8.8.8.8".to_owned(),
            running: false,
            output: Default::default(),
        }
    }
}

fn ping_stream(host: String) -> impl futures::Stream<Item = Message> {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    std::thread::spawn(move || {
        let mut child = match std::process::Command::new("ping")
            .arg(&host)
            .stdout(Stdio::piped())
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
            Message::PingStart => {
                if self.address.is_empty() || self.running {
                    return Task::none();
                }
                self.running = true;
                self.output = text_editor::Content::new();
                return Task::run(ping_stream(self.address.clone()), |m| m);
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
            .on_submit(Message::PingStart);

        let ping_btn = button(
            text(if self.running { "pinging..." } else { "ping" })
                .size(15)
                .center(),
        )
        .width(Length::Fixed(90.0))
        .on_press_maybe((!self.running).then_some(Message::PingStart));

        let output = text_editor(&self.output)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .placeholder("ping output...")
            .on_action(Message::PingEditorAction); // make read-only by ignoring edits

        let content = widget::column![
            row![input, space().width(8), ping_btn].align_y(Alignment::Center),
            space().height(8),
            output,
        ]
        .height(Length::Fill);

        let container = container(content).padding(12).height(Length::Fill);

        widget::column![
            row![
                go_back_button(13),
                space().width(16),
                title_text(self).align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center),
            space().height(10),
            container,
        ]
        .padding(20)
        .height(Length::Fill)
        .into()
    }
}
