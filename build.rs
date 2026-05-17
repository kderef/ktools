fn main() {
    if cfg!(target_os = "windows") {
        if std::env::var("PROFILE").unwrap() == "release" {
            static_vcruntime::metabuild();
            let mut res = winres::WindowsResource::new();

            res.set_icon("icon.ico");

            res.compile().unwrap();
        }
    }
}
