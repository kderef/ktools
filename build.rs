use std::process::Command;

#[cfg(windows)]
use winapi::um::winnt::{LANG_ENGLISH, SUBLANG_ENGLISH_US};

fn git_commit_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    // We ignore running this build script on dev builds for faster compile times.
    // On release, an icon is baked into the exe, as well as static vcruntime.

    let git_hash = git_commit_hash();

    // env!() vars
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    if cfg!(target_os = "windows") {
        if std::env::var("PROFILE").unwrap() == "release" {
            static_vcruntime::metabuild();

            // winres
            let language = winapi::um::winnt::MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US);

            let mut res = winres::WindowsResource::new();

            res.set_icon("icon.ico");
            res.set_language(language);

            res.compile().unwrap();
        }
    }
}
