use std::collections::HashMap;

use iced::{
    Alignment, Length,
    widget::{self, row, text},
};

use super::*;
use crate::Message;

static TASKS: &[(&str, fn() -> Result<String, String>)] = &[
    //
    ("hostname", fetch_hostname),
];

pub struct SystemInfo {
    info: HashMap<&'static str, Option<Result<String, String>>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            info: HashMap::from_iter(TASKS.iter().map(|(k, _)| (*k, None))),
        }
    }
}

impl Tool for SystemInfo {
    fn name(&self) -> &str {
        "System Information"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::vm()
    }
    fn background(&self) -> Color {
        rgb8(66, 123, 88)
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        Task::batch(TASKS.iter().map(|(k, f)| {
            Task::perform(async move { f() }, move |r| {
                Message::SystemInfoFetched(k, r)
            })
        }))
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SystemInfoFetched(key, result) => {
                if let Some(val) = self.info.get_mut(&key) {
                    *val = Some(result);
                }
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let mut rows = widget::column![];

        for (k, v) in &self.info {
            rows = rows.push(info_row(k, v));
        }

        let container = content_container(rows).padding(12);

        container.into()
    }
}

fn info_row<'a>(key: &str, value: &Option<Result<String, String>>) -> Element<'a, crate::Message> {
    let (value_text, value_color) = match value {
        None => ("Loading...".to_string(), rgb8(140, 140, 140)),
        Some(Ok(v)) => (v.clone(), rgb8(200, 200, 200)),
        Some(Err(e)) => (format!("ERROR: {e}"), rgb8(220, 10, 10)),
    };

    let mut row = row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(value_text.clone())
            .size(15)
            .width(Length::Fill)
            .color(value_color),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0]);

    if value.is_some() {
        row = row.push(copy_icon_btn(value_text.to_owned()));
    }

    row.into()
}

fn fetch_hostname() -> Result<String, String> {
    std::env::var("COMPUTERNAME").map_err(|e| e.to_string())
}
