# KTools
KTools is a GUI application that combines many smaller tools into one app.  
Examples of tools included are: **Password Generator** and **External IP Fetcher**

---
### OS Compatibility
I have only tested it on windows, but there is no OS specific code and it *should* work on Macos and Linux.

---

### How to install

Head on over to the [releases tab](https://github.com/kderef/ktools/releases) and select the latest version.
Then click *assets* and download the executable!

---

### Building from source

Building requires `Cargo` and `git` to be installed.

Run the following commands to build from source:
```sh
git clone https://github.com/kderef/ktools
cd ktools
cargo build --release
```

After it is done, the executable will be in `target/release`.
