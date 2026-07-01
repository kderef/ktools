//! This file contains the global `Message` type used in the app,
//! other sub-types of `Message` are converted into the global message here.

use std::net::IpAddr;

use crate::*;

/// Main message type used in the App.
/// It has a couple of generic messages such as `GoHome`
/// and a couple of `Tool`-specific messages such as `ExternalIpFetched()`
#[derive(Debug, Clone)]
pub enum Message {
    /// Ignored message, not handled.
    Ignore,

    /// Runs once when the window is opened
    Startup,

    /// Window message, handled by `WindowHandler`
    Window(window::Message),

    /// Data from `Tool::load_data()` was loaded.
    InitialDataLoaded(usize, Box<Self>),
    OpenURL(String),

    DownloadStart(String),
    /// Sent after `DownloadStart`
    DownloadStarted(usize),
    DownloadProgress(usize, download::Progress),
    DownloadFinished(usize, Result<Vec<u8>, download::DownloadError>),

    /// Something was selected in the sidebar
    SidebarOptionSelected(usize),

    /* Home page messages */
    PrimaryIPv4Loaded(Result<IpAddr, String>),

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
    SystemInfo(tool::sys_info::Message),

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

impl From<tool::sys_info::Message> for Message {
    fn from(value: tool::sys_info::Message) -> Self {
        Self::SystemInfo(value)
    }
}
