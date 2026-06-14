use crate::{ui::SidebarItem, *};

/// Main message type used in the App.
/// It has a couple of generic messages such as `GoHome`
/// and a couple of `Tool`-specific messages such as `ExternalIpFetched()`
#[derive(Debug, Clone)]
pub enum Message {
    /// Runs once when the window is opened
    Startup,
    Window(window::Message),

    OpenURL(String),

    /// Something was selected in the sidebar
    SidebarOption(SidebarItem),

    /* Home page messages */
    /// Go to index of App::tools
    ChooseTool(usize),
    GoHome,
    GoToSettings,
    Search(String),

    /* Generic messages */
    Refresh,
    CategorySelected(usize),
    TabSelected(usize),
    CopyToClipboard(String),
    TopTabSelected(usize),

    /* messages for applications */
    ApplicationOpen {
        cmd: &'static [&'static str],
        create_new_console: bool,
        elevate: bool,
    },

    /* messages for settings */
    SetTheme(tool::settings::ThemeSetting),
    ResetAllSettings,
    FetchedLatestGitTag(Result<String, String>),

    /* messages for netinfo */
    NetworkInterfacesFetched(Result<Vec<Adapter>, String>),

    /* messages for passgen */
    PasswordGenerator(tool::passgen::Message),

    /* messages for ext_ip */
    ExternalIpFetched(Result<tool::ext_ip::Object, String>),
    ExternalIpPick(tool::ext_ip::Api),

    /* messages for sys_info */
    SystemInfoFetched(&'static str, Result<tool::sys_info::SystemValue, String>),
    SystemInfoOpen(tool::sys_info::ProcessOpen),

    /* messages for ping */
    Ping(tool::ping::Message),
}

impl From<tool::ping::Message> for Message {
    fn from(value: tool::ping::Message) -> Self {
        Self::Ping(value)
    }
}

impl From<tool::passgen::Message> for Message {
    fn from(value: tool::passgen::Message) -> Self {
        Self::PasswordGenerator(value)
    }
}
