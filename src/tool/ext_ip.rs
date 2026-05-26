use iced::{
    Alignment, Length,
    widget::{self, button, row, space, text},
};

use crate::Message;
use serde_json::{Map, Value};

use super::*;

const IP_API_URL: &str = "http://ip-api.com/json";

pub type Object = Map<String, Value>;

#[derive(Default)]
pub struct ExternalIP {
    /// `None` when still loading, `Some` when loaded.
    response: Option<Result<Object, String>>,
}

impl Tool for ExternalIP {
    fn name(&self) -> &str {
        "External IP"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::broadcast()
    }
    fn background(&self) -> Color {
        rgb8(100, 100, 100)
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        fn get(url: &str) -> Result<Object, String> {
            ureq::get(url)
                .call()
                .and_then(|mut r| r.body_mut().read_to_string())
                .map_err(|e| e.to_string())
                .and_then(|t| serde_json::from_str::<Value>(&t).map_err(|e| e.to_string()))
                .and_then(|v| match v {
                    Value::Object(o) if !o.is_empty() => Ok(o),
                    _ => Err("Returned value was not an object.".to_owned()),
                })
        }

        Task::perform(async { get(IP_API_URL) }, crate::Message::ExternalIpFetched)
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            Message::ExternalIpFetched(response) => {
                self.response = Some(response);
            }
            Message::Refresh => {
                self.response = None;
                return self.on_activate();
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        let mut rows = widget::column![];

        match &self.response {
            None => {
                rows = rows.push(text("Loading...").size(15).style(text::secondary));
            }
            Some(Err(e)) => {
                rows = rows.push(text(format!("ERROR: {e}")).size(15).style(text::danger));
            }
            Some(Ok(val)) => {
                for (key, value) in format_obj(val).iter() {
                    rows = rows.push(info_row(key, value));
                }
            }
        }

        let container = content_container(rows).padding(12).height(Length::Fill);
        let go_back = go_back_button(13);
        let title = title_text(self);

        let mut col = widget::column![
            widget::row![go_back, space().width(16), title.align_y(Alignment::Center)]
                .align_y(Alignment::Center),
            space().height(10),
            container
        ];

        let bottom_row = row![
            button(text("refresh").size(24).center())
                .on_press(Message::Refresh)
                .width(Length::Fill),
            space().width(10),
            button(text("copy all").size(24).center())
                .width(Length::Fill)
                .on_press_with(|| {
                    let text = match &self.response {
                        Some(Ok(obj)) => obj_pretty(&format_obj(obj)),
                        _ => String::new(),
                    };
                    Message::CopyToClipboard(text)
                })
        ];

        col = col.push(space().height(20)).push(bottom_row);
        col.height(Length::Fill).padding(12).into()
    }
}

fn format_obj(obj: &Object) -> Object {
    let mut new = Object::new();

    if let Some(ip) = obj.get("query") {
        new.insert("IP Address".to_owned(), ip.clone());
    }

    for (k, v) in obj.iter() {
        match k.as_str() {
            "status" | "query" => {}
            _ => {
                new.insert(k.clone(), v.clone());
            }
        }
    }

    new
}

fn obj_pretty(obj: &Object) -> String {
    obj.iter()
        .map(|(k, v)| {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            format!("{k}: {val}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn info_row<'a>(key: &str, value: &Value) -> Element<'a, crate::Message> {
    let value_text = match value {
        Value::String(s) if !s.is_empty() => s.clone(),
        Value::Null | Value::String(_) => "-".to_owned(),
        other => other.to_string(),
    };

    let is_empty = value_text == "-";

    let mut row = row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .style(text::primary),
        text(value_text.clone()).size(15).width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0]);

    if !is_empty {
        row = row.push(copy_icon_btn(value_text));
    }

    row.into()
}
