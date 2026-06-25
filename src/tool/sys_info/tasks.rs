use super::*;

#[derive(Debug, Clone)]
pub enum Message {
    Fetched(FetchTask, Result<SystemValue, String>),
    OpenProcess(ProcessOpen),
    Refresh,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FetchTask {
    System,
    Hostname,
    Username,
    Cpu,
    GraphicsCard,
    Ram,
    Disks,
}
impl std::fmt::Display for FetchTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Tasks that are performed simultaneously in their own `Task`'s
impl FetchTask {
    pub const fn all() -> &'static [Self] {
        &[
            Self::System,
            Self::Hostname,
            Self::Username,
            Self::Cpu,
            Self::GraphicsCard,
            Self::Ram,
            Self::Disks,
        ]
    }
    pub const fn name(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Hostname => "Hostname",
            Self::Username => "Username",
            Self::Cpu => "CPU",
            Self::GraphicsCard => "Graphics Card",
            Self::Ram => "RAM",
            Self::Disks => "Disks",
        }
    }
    pub const fn action(self) -> fn() -> Result<SystemValue, String> {
        match self {
            Self::System => fetch_os,
            Self::Hostname => fetch_hostname,
            Self::Username => fetch_username,
            Self::Cpu => fetch_cpu,
            Self::GraphicsCard => fetch_graphics_card,
            Self::Ram => fetch_ram,
            Self::Disks => fetch_disks,
        }
    }
}

fn fetch_hostname() -> Result<SystemValue, String> {
    System::host_name()
        .map(SystemValue::Text)
        .ok_or("unavailable".to_string())
}

fn fetch_username() -> Result<SystemValue, String> {
    std::env::var("USERNAME")
        .or(std::env::var("USER"))
        .map(SystemValue::Text)
        .map_err(|e| e.to_string())
}

fn fetch_cpu() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let cpu = cpus.first().ok_or("No cpu was found".to_owned())?;

    Ok(SystemValue::Cpu {
        brand: cpu.brand().trim().to_owned(),
        frequency: cpu.frequency() as f32 / 1000.0,
        cores: cpus.len(),
    })
}

fn fetch_ram() -> Result<SystemValue, String> {
    let mut sys = System::new();
    sys.refresh_memory();

    Ok(SystemValue::Memory {
        total: Bytes(sys.total_memory()),
        free: Bytes(sys.free_memory()),
        used: Bytes(sys.used_memory()),
    })
}

fn fetch_os() -> Result<SystemValue, String> {
    Ok(SystemValue::System {
        description_long: System::long_os_version().ok_or("unknown OS type".to_owned())?,
        description_short: System::os_version().ok_or("Unknown OS type".to_owned())?,
        kernel_version: System::kernel_long_version(),
        arch: System::cpu_arch(),
    })
}

fn fetch_disks() -> Result<SystemValue, String> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    let disks = disks
        .iter()
        .map(|d| Disk {
            name: d.name().to_string_lossy().to_string(),
            mount: d.mount_point().to_string_lossy().to_string(),
            total: Bytes(d.total_space()),
            free: Bytes(d.available_space()),
            used: Bytes(d.total_space() - d.available_space()),
        })
        .collect();

    Ok(SystemValue::Disks(disks))
}

fn fetch_graphics_card() -> Result<SystemValue, String> {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIFactory};

        let factory: IDXGIFactory = CreateDXGIFactory().map_err(|e| e.to_string())?;

        let adapter = factory.EnumAdapters(0).map_err(|e| e.to_string())?;

        let desc = adapter.GetDesc().map_err(|e| e.to_string())?;

        let name = String::from_utf16_lossy(&desc.Description);

        Ok(SystemValue::Text(name.trim_end_matches('\0').to_string()))
    }
}
