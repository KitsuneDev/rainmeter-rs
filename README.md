# rainmeter

## Rainmeter Rust Plugin Interface

A (probably)-safe, high-level Rust wrapper around RainmeterΓÇÖs C/C++ plugin API.
(I don't write too much Rust, so this is a work in progress. Don't judge me too harshly, please!)

Please note this is platform-specific to Windows, as Rainmeter is a Windows-only application.

**For more information, please refer to the [Rainmeter C++ API Overview](https://docs.rainmeter.net/developers/plugin/cpp/) (for the implementable functions) and the [Rainmeter C++ API Reference](https://docs.rainmeter.net/developers/plugin/cpp/api/) (Which is accessed via the `RainmeterContext` arguments).**

### Features

- Write your Rainmeter plugins in idiomatic Rust, with traits!!
- Ergonomic Rust methods for reading measure options (`ReadString`, `ReadFormula`, etc.)
- Typed getters for skin data (`measure name`, `skin path`, `HWND`, etc.)
- Safe logging and panic-handling in your plugin entry points
- Simple `RainmeterPlugin` trait and `declare_plugin!` macro to expose plugins

### Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
rainmeter = "0.1"

[lib]
crate-type = ["cdylib"]
```

### Usage

```rust
use rainmeter::{RainmeterContext, RainmeterPlugin, RmLogLevel, declare_plugin};

#[derive(Default)]
struct MyPlugin;

impl RainmeterPlugin for MyPlugin {
    fn initialize(&mut self, rm: RainmeterContext) {
        let val = rm.read_double("Value", 1.0);
        rm.log(RmLogLevel::LogNotice, &format!("Value = {}", val));
    }

    fn reload(&mut self, rm: RainmeterContext, _max: &mut f64) {}
    fn update(&mut self, _rm: RainmeterContext) -> f64 { 0.0 }
    fn finalize(&mut self, _rm: RainmeterContext) {}
    fn get_string(&mut self, _rm: RainmeterContext) -> Option<String> { // Optional
        None
    }
    fn execute_bang(&mut self, _rm: RainmeterContext, _args: &str) {} // Optional
}

declare_plugin!(crate::MyPlugin);
```

License: LGPL-3.0-or-later
