use iced::{
    Alignment, Length, Theme,
    widget::{self, button, pick_list, row, space, text},
};

use crate::Message;
use serde_json::{Map, Value};

use super::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Api {
    #[default]
    IpApi,
    Ipify,
    IpApiCo,
}
impl ToString for Api {
    fn to_string(&self) -> String {
        self.url().to_owned()
    }
}

impl Api {
    const fn url(self) -> &'static str {
        match self {
            Self::IpApi => "http://ip-api.com/json",
            Self::Ipify => "https://api.ipify.org/?format=json",
            Self::IpApiCo => "https://ipapi.co/json",
        }
    }
    const fn all() -> &'static [Self] {
        &[Self::IpApi, Self::Ipify, Self::IpApiCo]
    }
}

pub type Object = Map<String, Value>;

#[derive(Default)]
pub struct ExternalIP {
    /// `None` when still loading, `Some` when loaded.
    response: Option<Result<Object, String>>,

    api: Api,
}

impl Tool for ExternalIP {
    fn name(&self) -> &str {
        "External IP"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::broadcast()
    }
    fn background(&self, theme: &Theme) -> Color {
        // rgb8(100, 100, 100)
        theme.extended_palette().secondary.base.color
    }
    fn on_activate(&mut self) -> Task<crate::Message> {
        fn get(api: Api) -> Result<Object, String> {
            minreq::get(api.url())
                .send()
                .map_err(|e| e.to_string())?
                .as_str()
                .map_err(|e| e.to_string())
                .and_then(|t| serde_json::from_str::<Value>(&t).map_err(|e| e.to_string()))
                .and_then(|v| match v {
                    Value::Object(o) if !o.is_empty() => Ok(o),
                    _ => Err("Returned value was not an object.".to_owned()),
                })
        }

        let api = self.api;
        Task::perform(async move { get(api) }, crate::Message::ExternalIpFetched)
    }

    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            Message::ExternalIpFetched(response) => {
                self.response = Some(response);
            }
            Message::ExternalIpPick(api) => {
                self.api = api;
                return self.on_activate();
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

        let top_row = row![pick_list(
            Api::all(),
            Some(&self.api),
            Message::ExternalIpPick
        )];

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

        widget::column![top_row, container, bottom_row]
            .height(Length::Fill)
            .padding(12)
            .spacing(20)
            .into()
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
        .map(|(k, v)| format!("{k}: {}", v.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn info_row<'a>(key: &str, value: &Value) -> Element<'a, crate::Message> {
    let mut is_empty = false;

    let value_text = match value {
        Value::String(s) if !s.is_empty() => s.clone(),
        Value::Null | Value::String(_) => {
            is_empty = true;
            "-".to_owned()
        }
        other => other.to_string(),
    };

    let mut row = row![
        text(key.to_string())
            .size(15)
            .width(Length::Fixed(160.0))
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color)
            }),
        text(value_text.clone()).size(15).width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0]);

    if !is_empty {
        row = row.push(copy_icon_btn(value_text));
    }

    row = row.push(space().width(20));

    row.into()
}
