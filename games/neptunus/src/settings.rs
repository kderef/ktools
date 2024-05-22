pub struct Settings {
    /// 0..=100
    pub volume: u32,
    /// 0..=500
    pub fps_limit: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume: 75,
            fps_limit: 120,
        }
    }
}