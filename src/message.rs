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

    /* Home page messages */
    /// Go to index of App::tools
    ChooseTool(usize),
    GoHome,
    GoToSettings,

    /* Generic messages */
    Refresh,
    CategorySelected(usize),
    TabSelected(usize),
    CopyToClipboard(String),
    TopTabSelected(usize),

    /* messages for settings */
    SetTheme(tool::settings::ThemeSetting),
    ResetToolOrder,
    MoveToolUp(usize),
    MoveToolDown(usize),
    ResetAllSettings,

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
    PingStart(Option<String>),
    PingCancel,
    PingDefaultGateway,
    PingAddressChanged(String),
    PingEditorAction(text_editor::Action),
    PingToggleCustom,
    PingOutput(String),
    PingDone,
}
