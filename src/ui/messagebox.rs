pub fn messagebox_err(title: &str, message: &str) {
    #[cfg(windows)]
    unsafe {
        use windows::{
            Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW},
            core::HSTRING,
        };

        let style = MB_ICONERROR | MB_OK;

        let title = HSTRING::from(title);
        let message = HSTRING::from(message);

        let _result = MessageBoxW(None, &message, &title, style);
    }
}
