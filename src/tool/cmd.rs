use std::os::windows::process::CommandExt;

use super::*;

#[derive(Default)]
pub struct CMD;

impl Tool for CMD {
    fn name(&self) -> &'static str {
        "CMD"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::terminal()
    }
    fn no_view(&self) -> bool {
        true
    }

    fn background(&self) -> Color {
        rgb(0.08, 0.08, 0.08)
    }
    fn text_color(&self) -> Color {
        rgb(0.9, 0.9, 0.9)
    }

    fn update(&mut self, _message: crate::Message) -> Task<crate::Message> {
        unreachable!()
    }

    fn view(&self) -> Element<'_, crate::Message> {
        unreachable!()
    }

    fn on_select(&mut self) {
        const CREATE_NEW_CONSOLE: u32 = 0x00000010;

        let _child = std::process::Command::new("cmd.exe")
            .creation_flags(CREATE_NEW_CONSOLE)
            // .args(["/C", "start"])
            .spawn();
    }
}
