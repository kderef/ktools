fn main() {
    // We ignore running this build script on dev builds for faster compile times.
    // On release, an icon is baked into the exe, as well as static vcruntime.

    if cfg!(target_os = "windows") {
        if std::env::var("PROFILE").unwrap() == "release" {
            static_vcruntime::metabuild();
            let mut res = winres::WindowsResource::new();

            res.set_icon("icon.ico");
            // res.set_version_info(winres::VersionInfo::PRODUCTVERSION, version);

            res.compile().unwrap();
        }
    }
}
