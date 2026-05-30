# KTools
KTools is a GUI application that combines many smaller tools into one app.  
Examples of tools included are: **Password Generator** and **External IP Fetcher**

### How to install
Head on over to the [releases tab](https://github.com/kderef/ktools/releases) and select the latest version.
Then click *assets* and download the executable!

---

### License
This project is licensed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).  
A link to this source code is included inside the compiled application.  

This project includes a modified version of the [ipconfig](https://github.com/liranringel/ipconfig) crate, originally licensed under MIT.  
See [ipconfig/LICENSE](ipconfig/LICENSE) for details.  

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
