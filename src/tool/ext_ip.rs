use iced::{
    Alignment, Length,
    widget::{self, row, space, text},
};

use crate::Message;

use super::*;

const IPV4_URL: &str = "https://api.ipify.org";
const IPV6_URL: &str = "https://api6.ipify.org";

async fn get(url: &str) -> Result<String, String> {
    reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}
fn info_row<'a>(
    label: &'a str,
    response: &Option<Result<String, String>>,
) -> Element<'a, crate::Message> {
    let (res_text, res_color) = match response {
        None => ("Loading...".to_string(), rgb8(140, 140, 140)),
        Some(Ok(ip)) => (ip.clone(), rgb8(180, 180, 180)),
        Some(Err(e)) => (format!("ERROR: {e}"), rgb8(180, 10, 10)),
    };

    row![
        text(label)
            .size(15)
            .width(Length::Fixed(160.0))
            .color(rgb8(160, 160, 160)),
        text(res_text.to_string())
            .size(15)
            .width(Length::Fill)
            .color(res_color),
        copy_icon_btn(res_text.to_string()),
    ]
    .align_y(Alignment::Center)
    .padding([5, 0])
    .into()
}

#[derive(Default)]
pub struct ExternalIP {
    ipv4: Option<Result<String, String>>,
    ipv6: Option<Result<String, String>>,
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
        Task::perform(
            async { (get(IPV4_URL).await, get(IPV6_URL).await) },
            crate::Message::ExternalIpFetched,
        )
    }
    fn update(&mut self, message: crate::Message) -> Task<crate::Message> {
        match message {
            Message::ExternalIpFetched((ipv4, ipv6)) => {
                self.ipv4 = Some(ipv4);
                self.ipv6 = Some(ipv6);
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        let content = widget::column![info_row("IPv4", &self.ipv4), info_row("IPv6", &self.ipv6)];

        let container = content_container(content).padding(12);

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
