use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Re-run this build script if anything in libs/ changes
    println!("cargo:rerun-if-changed=native");

    // e.g. "x86_64-pc-windows-msvc" or "i686-pc-windows-msvc"
    let target = env::var("TARGET").unwrap();
    // Decide folder based on architecture substring
    let arch_dir = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("Unsupported TARGET for Rainmeter crate: {}", target);
    };

    // Tell rustc where to find the .lib
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let lib_path = Path::new(&manifest_dir)
        .join("native")
        .join("sdk")
        .join("API")
        .join(arch_dir);
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Link the import‑library.  Since Rainmeter.lib is an import lib for a DLL,
    // we use `dylib` here.  If it were a truly static library, use `static` instead.
    println!("cargo:rustc-link-lib=dylib=Rainmeter");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("native/wrapper.h")
        // parse as C++17 so default args and inline statics (if any slipped through) are ok
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        // make sure the compiler actually defines those UNICODE macros
        .clang_arg("-DUNICODE")
        .clang_arg("-D_UNICODE")
        // target Windows MSVC (so it picks up the right ABI/macros)
        .clang_arg("--target=x86_64-pc-windows-msvc")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
