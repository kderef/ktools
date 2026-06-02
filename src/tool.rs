pub mod cmd;
pub mod ext_ip;
pub mod netinfo;
pub mod passgen;
pub mod ping;
pub mod settings;
pub mod sys_info;

use crate::Message;
pub use crate::base::*;

use iced::{Task, Theme};

pub use iced::{Color, Element, widget::Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Category {
    Application,
    Utility,
    System,
    Network,
}

impl Category {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Application => "applications",
            Self::Utility => "utility",
            Self::System => "system",
            Self::Network => "network",
        }
    }
    pub const fn all() -> &'static [Self] {
        &[
            Self::Application,
            Self::Utility,
            Self::System,
            Self::Network,
        ]
    }
}

#[derive(Debug, Clone)]
pub enum ToolMessage {
    // Cmd
    Ping(ping::Message),
    PassGen(passgen::Message),
    NetInfo(netinfo::Message),
    ExtIp(ext_ip::Message),
    SysInfo(sys_info::Message),
    Settings(settings::Message),
}

pub struct ToolInfo {
    pub title: &'static str,
    pub icon: fn() -> Text<'static>,
    pub category: Category,
    pub background: fn(&Theme) -> Color,
    pub no_view: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Tool {
    Cmd(cmd::CMD),
    Ping(ping::Ping),
    PassGen(passgen::PasswordGenerator),
    NetInfo(netinfo::NetworkInfo),
    ExtIp(ext_ip::ExternalIP),
    SysInfo(sys_info::SystemInfo),

    Settings(settings::Settings),
}

impl Tool {
    pub const fn info(&self) -> ToolInfo {
        match self {
            Self::Cmd(_) => ToolInfo {
                title: "CMD",
                icon: icon_font::terminal,
                category: Category::Application,
                background: cmd::background,
                no_view: true,
            },
            Self::Ping(_) => ToolInfo {
                title: "Ping",
                icon: icon_font::debug_disconnect,
                category: Category::Network,
                background: ping::background,
                no_view: false,
            },
            Self::PassGen(_) => ToolInfo {
                title: "Password Generator",
                icon: icon_font::lock,
                category: Category::Utility,
                background: passgen::background,
                no_view: false,
            },
            Self::Settings(_) => unreachable!(),
            Self::NetInfo(_) => ToolInfo {
                title: "Network Information",
                icon: icon_font::globe,
                category: Category::Network,
                background: netinfo::background,
                no_view: false,
            },
            Self::ExtIp(_) => ToolInfo {
                title: "External IP",
                icon: icon_font::broadcast,
                category: Category::Network,
                background: ext_ip::background,
                no_view: false,
            },
            Self::SysInfo(_) => ToolInfo {
                title: "System Information",
                icon: icon_font::vm,
                category: Category::System,
                background: sys_info::background,
                no_view: false,
            },
        }
    }
    pub fn on_activate(&mut self) -> Task<Message> {
        match self {
            Self::Cmd(c) => return c.on_activate().map(Into::into),
            Self::Ping(_) => {}
            Self::PassGen(p) => return p.on_activate().map(Into::into),
            Self::NetInfo(n) => return n.on_activate().map(Into::into),
            Self::ExtIp(e) => return e.on_activate().map(Into::into),
            Self::SysInfo(s) => return s.on_activate().map(Into::into),
            Self::Settings(_) => {}
        }

        Task::none()
    }
    pub fn update(&mut self, message: ToolMessage) -> Task<Message> {
        match (self, message) {
            (Self::Cmd(_), _) => {}
            (Self::Ping(p), ToolMessage::Ping(m)) => {
                return p.update(m).map(Into::into);
            }
            (Self::PassGen(p), ToolMessage::PassGen(m)) => {
                return p
                    .update(m)
                    .map(|m| Message::ToolMessage(ToolMessage::PassGen(m)));
            }
            (Self::NetInfo(t), ToolMessage::NetInfo(m)) => return t.update(m).map(Into::into),
            (Self::ExtIp(t), ToolMessage::ExtIp(m)) => return t.update(m).map(Into::into),
            (Self::SysInfo(t), ToolMessage::SysInfo(m)) => return t.update(m).map(Into::into),
            (Self::Settings(t), ToolMessage::Settings(m)) => return t.update(m).map(Into::into),
            _ => {}
        }
        Task::none()
    }
    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Self::Cmd(_) => unreachable!(),
            Self::Ping(t) => t.view().map(Into::into),
            Self::PassGen(t) => t.view().map(Into::into),
            Self::NetInfo(t) => t.view().map(Into::into),
            Self::ExtIp(t) => t.view().map(Into::into),
            Self::SysInfo(t) => t.view().map(Into::into),
            Self::Settings(_) => unreachable!(),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Cmd(cmd::CMD::default()),
            Self::Ping(ping::Ping::default()),
            Self::PassGen(passgen::PasswordGenerator::default()),
            Self::NetInfo(netinfo::NetworkInfo::default()),
            Self::ExtIp(ext_ip::ExternalIP::default()),
            Self::SysInfo(sys_info::SystemInfo::default()),
            // Self::Settings is skipped!
        ]
    }
}
