# rainmeter-sys

Low-level, unsafe Rust FFI bindings to the Rainmeter C/C++ plugin API. This crate is generated with bindgen at build time and links against the official Rainmeter import library bundled in this repository.

If you want to write Rainmeter plugins in idiomatic, safe Rust, you most likely want the high-level wrapper crate instead: `rainmeter` (`../rainmeter-rs`) — see its README and docs: https://docs.rs/crate/rainmeter/latest

## What this crate provides
- Raw `extern "C"` bindings to the functions and types from Rainmeter's C++ API headers (via `bindgen`).
- Automatic linking to the appropriate Rainmeter import library (`Rainmeter.lib`) for your target architecture.
- No opinionated abstractions; you are expected to use `unsafe` and manage ABI details yourself.

## Platform and toolchain support
- Windows only (Rainmeter is Windows-only).
- MSVC toolchain: `x86_64-pc-windows-msvc` (x64) and `i686-pc-windows-msvc` (x86) targets are intended to work.
- Rust: stable Rust is fine.

## How it works
- The Rainmeter SDK headers and import libraries are vendored under `native/sdk`.
- A tiny wrapper header (`native/wrapper.h`) sets up `UNICODE`/`_UNICODE`, defines `LIBRARY_EXPORTS`, and includes `Windows.h` and `sdk/API/RainmeterAPI.h`.
- `build.rs` runs `bindgen` against that wrapper and writes Rust bindings to `$OUT_DIR/bindings.rs`, which is then included by `src/lib.rs`.
- The build script also emits `cargo:rustc-link-search` for `native/sdk/API/{x64|x86}` and `cargo:rustc-link-lib=dylib=Rainmeter` so your plugin links to the Rainmeter host at runtime.

Repo layout (selected):
- `rainmeter-sys/src/lib.rs` — includes the generated bindings
- `rainmeter-sys/build.rs` — bindgen + link directives
- `rainmeter-sys/native/sdk` — vendored Rainmeter SDK (headers + `.lib` files)
- `rainmeter-sys/native/wrapper.h` — bindgen entry header

## Prerequisites
Because the bindings are generated at build time, you need a C/C++ toolchain and libclang available.

- Visual Studio Build Tools (or full Visual Studio) with the "Desktop development with C++" workload and a recent Windows SDK.
- Rust MSVC toolchain (`rustup toolchain install stable-x86_64-pc-windows-msvc`). For 32-bit builds also install `i686-pc-windows-msvc`.
- LLVM with libclang (recommended 15+). If `bindgen` cannot find `libclang`, set the `LIBCLANG_PATH` environment variable to the folder that contains `libclang.dll`.

Example for PowerShell (adjust version/path):
```powershell
$env:LIBCLANG_PATH = "C:\\Program Files\\LLVM\\bin"
```

## Installation
Add to your `Cargo.toml` if using directly (advanced/FFI-only use cases):
```toml
[dependencies]
rainmeter-sys = { version = "0.1", package = "rainmeter-sys" }
```
Within this workspace, other crates depend on it by path.

Most users should instead depend on the safe wrapper:
```toml
[dependencies]
rainmeter = "0.1"
```

## Building
- 64-bit (default on a 64-bit toolchain):
```powershell
cargo build --release
```
- 32-bit (if you need a 32-bit plugin):
```powershell
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
```

The build script chooses the correct `native/sdk/API/{x64|x86}` library directory based on your target triple.

## Using the raw bindings
The raw API mirrors the C/C++ Rainmeter API and requires `unsafe`. Names and signatures come directly from the Rainmeter headers. A very rough sketch (do not copy blindly):
```rust
use rainmeter_sys as sys;

unsafe fn example_logging(rm: sys::LPVOID) {
    // Function names/types are generated from the C++ headers and may differ.
    // Refer to the generated docs or inspect `OUT_DIR/bindings.rs` when building.
    let msg = widestring::U16CString::from_str_unchecked("Hello from Rust (sys)!");
    // e.g. sys::RmLog(rm, sys::LOG_NOTICE, msg.as_ptr());
}
```
For a practical, safe approach with traits and ergonomic helpers, use the `rainmeter` crate in this repo.

## Troubleshooting
- bindgen/libclang not found:
  - Install LLVM and set `LIBCLANG_PATH` to the directory containing `libclang.dll`.
- Linker cannot find `Rainmeter.lib`:
  - Ensure you are building for `*-pc-windows-msvc` and not `gnu`.
  - Make sure your target architecture matches the selected library directory (x64 vs x86).
- Windows SDK headers not found:
  - Install the C++ Desktop workload and a recent Windows SDK via Visual Studio Installer.

## Related links
- Rainmeter C++ API Overview: https://docs.rainmeter.net/developers/plugin/cpp/
- Rainmeter C++ API Reference: https://docs.rainmeter.net/developers/plugin/cpp/api/
- High-level Rust crate (`rainmeter`): https://docs.rs/crate/rainmeter/latest

## License
- Rust bindings in this crate: LGPL-3.0-or-later (same as the rest of this repository).
- Rainmeter SDK headers and import libraries are copyright © The Rainmeter Team and subject to their respective licenses.

