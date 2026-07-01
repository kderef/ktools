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

pub fn messagebox_yesno(title: &str, message: &str) -> bool {
    #[cfg(windows)]
    unsafe {
        use windows::{
            Win32::UI::WindowsAndMessaging::{IDYES, MB_ICONINFORMATION, MB_YESNO, MessageBoxW},
            core::HSTRING,
        };

        let style = MB_ICONINFORMATION | MB_YESNO;

        let title = HSTRING::from(title);
        let message = HSTRING::from(message);

        let result = MessageBoxW(None, &message, &title, style);

        return result == IDYES;
    }
}
