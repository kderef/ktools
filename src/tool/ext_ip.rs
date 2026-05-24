use iced::{
    Alignment, Length,
    widget::{self, row, space, text},
};
use json::JsonValue;

use crate::Message;

use super::*;

const IP_API_URL: &str = "http://ip-api.com/json";

#[derive(Default)]
pub struct ExternalIP {
    response: Option<Result<json::object::Object, String>>,
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
        fn get(url: &str) -> Result<json::JsonValue, String> {
            ureq::get(url)
                .call()
                .and_then(|mut r| r.body_mut().read_to_string())
                .map_err(|e| e.to_string())
                .and_then(|t| json::parse(&t).map_err(|e| e.to_string()))
                .map_err(|e| e.to_string())
        }

        Task::perform(
            async {
                match get(IP_API_URL) {
                    Ok(JsonValue::Object(o)) if !o.is_empty() => Ok(o),
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
                        .color(rgb8(160, 20, 20)),
                );
            }
            Some(Ok(val)) => {
                let mut val = val.clone();

                val.remove("status");
                if let Some(v) = val.remove("query") {
                    rows = rows.push(info_row("IPv4", &v));
                }

                for (key, value) in val.iter() {
                    match key {
                        "status" => {}
                        key => {
                            rows = rows.push(info_row(key, value));
                        }
                    }
                }
            }
        }

        let container = content_container(rows).padding(12);

        let go_back = go_back_button(13);
        let title = title_text(self);

        widget::column![
            widget::row![go_back, space().width(16), title.align_y(Alignment::Center)]
                .align_y(Alignment::Center),
            space().height(10),
            container
        ]
        .padding(12)
        .into()
    }
}

fn info_row<'a>(key: &str, value: &JsonValue) -> Element<'a, crate::Message> {
    let value_text = if value.is_empty() {
        "-".to_string()
    } else {
        value.to_string()
    };

    row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(value_text.clone()).size(15).width(Length::Fill),
        // .color(res_color),
        copy_icon_btn(value_text),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0])
    .into()
}
