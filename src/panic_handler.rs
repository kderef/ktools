//! Handles the global panic, showing a messagebox instead of crashing

use std::panic::PanicHookInfo;

use crate::ui;

pub fn handle_panic(info: &PanicHookInfo) {
    #[cfg(debug_assertions)]
    eprintln!("FATAL APP ERROR: {info:?}");

    let location_string = match info.location() {
        Some(loc) => format!("{}:{}:{}", loc.file(), loc.line(), loc.column()),
        None => "Unknown location".to_string(),
    };

    let message = format!(
        "Fatal Error occurred at location {location_string}\nMessage: {}",
        info.payload_as_str().unwrap_or("no message")
    );

    ui::messagebox_err("KTools FATAL ERROR", &message);
    std::process::exit(1);
}
