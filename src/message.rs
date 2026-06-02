use crate::*;

/// Only message type used in the App.
/// It has a couple of generic messages such as `GoHome`
/// and a couple of `Tool`-specific messages such as `ExternalIpFetched()`
#[derive(Debug, Clone)]
pub enum Message {
    /// Runs once when the window is opened
    Startup,
    Window(window::Message),

    OpenURL(&'static str),

    ToolMessage(tool::ToolMessage),

    ResetToolOrder,
    MoveToolUp(usize),
    MoveToolDown(usize),

    /* Home page messages */
    /// Go to index of App::tools
    ChooseTool(usize),
    GoHome,
    GoToSettings,
    Search(String),
    ResetAllSettings,

    /* Generic messages */
    Refresh,
    CategorySelected(usize),
    TabSelected(usize),
    CopyToClipboard(String),
    TopTabSelected(usize),
    /* messages for ext_ip */
}

impl From<tool::sys_info::Message> for Message {
    fn from(value: tool::sys_info::Message) -> Self {
        Self::ToolMessage(ToolMessage::SysInfo(value))
    }
}

impl From<tool::ext_ip::Message> for Message {
    fn from(value: tool::ext_ip::Message) -> Self {
        Self::ToolMessage(ToolMessage::ExtIp(value))
    }
}

impl From<tool::settings::Message> for Message {
    fn from(value: tool::settings::Message) -> Self {
        Self::ToolMessage(ToolMessage::Settings(value))
    }
}

impl From<tool::netinfo::Message> for Message {
    fn from(value: tool::netinfo::Message) -> Self {
        Self::ToolMessage(ToolMessage::NetInfo(value))
    }
}
impl From<tool::passgen::Message> for Message {
    fn from(value: tool::passgen::Message) -> Self {
        Self::ToolMessage(ToolMessage::PassGen(value))
    }
}

impl From<tool::ping::Message> for Message {
    fn from(value: tool::ping::Message) -> Self {
        Self::ToolMessage(ToolMessage::Ping(value))
    }
}
