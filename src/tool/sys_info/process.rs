use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ProcessOpen {
    ConfigPanel,
    Printers,
    AdminTools,
    Features,
    ComputerManagement,
    PowerOptions,
    DeviceManager,
    InstalledApps,
}

impl ProcessOpen {
    pub const fn command(self) -> &'static [&'static str] {
        match self {
            Self::ConfigPanel => &["control"],
            Self::Printers => &["explorer", "ms-settings:printers"],
            Self::AdminTools => &["control", "/name", "Microsoft.AdministrativeTools"],
            Self::Features => &["rundll32", "shell32.dll,Control_RunDLL", "appwiz.cpl,,2"],
            // Self::Features => &["control", "appwiz.cpl"],
            Self::ComputerManagement => &["cmd", "/C", "compmgmt.msc"],
            Self::PowerOptions => &["explorer", "ms-settings:powersleep"],
            Self::DeviceManager => &["cmd", "/C", "devmgmt.msc"],
            Self::InstalledApps => &["explorer", "ms-settings:appsfeatures"],
        }
    }
    pub fn icon(self) -> Text<'static> {
        match self {
            Self::ConfigPanel => icon_font::settings_gear(),
            Self::Printers => icon_font::preview(),
            Self::AdminTools => icon_font::account(),
            Self::Features => icon_font::tools(),
            Self::ComputerManagement => icon_font::vm(),
            Self::PowerOptions => icon_font::plug(),
            Self::DeviceManager => icon_font::server(),
            Self::InstalledApps => icon_font::multiple_windows(),
        }
    }
}
