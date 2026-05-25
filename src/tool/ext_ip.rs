use iced::{
    Alignment, Length,
    widget::{self, button, row, space, text},
};

use crate::Message;
use serde_json::Value;

use super::*;

const IP_API_URL: &str = "http://ip-api.com/json";

#[derive(Default)]
pub struct ExternalIP {
    response: Option<Result<serde_json::Map<String, Value>, String>>,
}

impl Tool for ExternalIP {
    fn name(&self) -> &str {
        "External IP"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::broadcast()
    }
    fn background(&self) -> Color {
        rgb8(100, 100, 100) // TODO
    }
    fn text_color(&self) -> Color {
        rgb(0.95, 0.95, 0.95)
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        fn get(url: &str) -> Result<serde_json::Value, String> {
            ureq::get(url)
                .call()
                .and_then(|mut r| r.body_mut().read_to_string())
                .map_err(|e| e.to_string())
                .and_then(|t| serde_json::from_str(&t).map_err(|e| e.to_string()))
        }

        Task::perform(
            async {
                match get(IP_API_URL) {
                    Ok(serde_json::Value::Object(o)) if !o.is_empty() => Ok(o),
                    Ok(_) => Err("Returned value was not an object.".to_owned()),
                    Err(e) => Err(e),
                }
            },
            crate::Message::ExternalIpFetched,
        )
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
                rows = rows.push(text("Loading...").size(15).color(rgb8(140, 140, 140)));
            }
            Some(Err(e)) => {
                rows = rows.push(
                    text(format!("ERROR: {e}"))
                        .size(15)
                        .color(rgb8(220, 20, 20)),
                );
            }
            Some(Ok(val)) => {
                let val = format_obj(val);

                for (key, value) in val.iter() {
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
                        Some(Ok(obj)) => obj_pretty(&format_obj(&obj)),
                        _ => "".to_string(),
                    };

                    Message::CopyToClipboard(text)
                })
        ];

        col = col.push(space().height(20)).push(bottom_row);

        col.height(Length::Fill).padding(12).into()
    }
}

fn format_obj(obj: &Object) -> Object {
    let mut new = Object::with_capacity(obj.len());

    if let Some(ip) = obj.get("query") {
        new.insert("IP Address", ip.to_owned());
    }

    for (k, v) in obj.iter() {
        match k {
            "status" | "query" => {}
            k => {
                new.insert(k, v.clone());
            }
        }
    }

    new
}

fn info_row<'a>(key: &str, value: &JsonValue) -> Element<'a, crate::Message> {
    let value_empty = value.is_empty();

    let value_text = if value_empty {
        "-".to_string()
    } else {
        value.to_string()
    };

    let mut row = row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(value_text.clone()).size(15).width(Length::Fill),
        // .color(res_color),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0]);

    if !value_empty {
        row = row.push(copy_icon_btn(value_text));
    }

    row.into()
}
