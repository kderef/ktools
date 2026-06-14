use std::{path::Path, process::Command};

#[cfg(windows)]
use winapi::um::winnt::{LANG_ENGLISH, SUBLANG_ENGLISH_US};

fn get_build_date() -> String {
    #[cfg(windows)]
    String::from_utf8(
        Command::new("cmd")
            .args(["/C", "echo", "%DATE%"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
}

fn git_commit_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap()
}

fn write_icon_rgba<P: AsRef<Path>>(input: P, output: P) {
    let bytes = std::fs::read(input).unwrap();

    let cursor = std::io::Cursor::new(bytes);
    let dir = ico::IconDir::read(cursor).unwrap();
    let entry = dir.entries()[0].decode().unwrap();

    println!("cargo:rustc-env=ICON_RGBA_WIDTH={}", entry.width());
    println!("cargo:rustc-env=ICON_RGBA_HEIGHT={}", entry.height());

    std::fs::write(output, entry.rgba_data()).unwrap();
}

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let is_release = profile == "release";

    // We ignore running this build script on dev builds for faster compile times.
    // On release, an icon is baked into the exe, as well as static vcruntime.

    let git_hash = git_commit_hash();
    let build_date = get_build_date();

    // env!() vars
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    // set icon
    println!("cargo:rerun-if-changed=icon.ico");
    if is_release {
        write_icon_rgba("icon.ico", "icon_raw_rgba");
    }

    if is_release {
        #[cfg(windows)]
        {
            static_vcruntime::metabuild();

            // winres
            let language = winapi::um::winnt::MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US);

            let mut res = winres::WindowsResource::new();

            println!("cargo:warn={res:?}");

            res.set_icon("icon.ico");
            res.set_language(language);

            // TODO: winres is supposed to grab from package.metadata.winres but it is not.
            res.set("LegalCopyright", "Copyright © 2026 Kian Heitkamp");
            res.set(
                "FileDescription",
                "App that makes getting system information easy",
            );

            res.compile().unwrap();
        }
    }
}
