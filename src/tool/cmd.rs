use std::os::windows::process::CommandExt;

use super::*;

#[derive(Default, Serialize, Deserialize)]
pub struct CMD;

pub const fn background(_theme: &Theme) -> Color {
    rgb(0.08, 0.08, 0.08)
}
impl CMD {
    pub fn on_activate(&mut self) -> Task<crate::Message> {
        const CREATE_NEW_CONSOLE: u32 = 0x00000010;

        let _child = std::process::Command::new("cmd.exe")
            .creation_flags(CREATE_NEW_CONSOLE)
            // .args(["/C", "start"])
            .spawn();

        Task::none()
    }
}
