extern crate bindgen;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::result::Result;
use bindgen::{Bindings,builder};

#[cfg(all(target_os="macos", target_arch="x86_64"))]
fn generate_bindings(target_header: &Path) -> Result<Bindings,()> {
    builder()
        .header(target_header.display().to_string())
        .clang_arg("--target=x86_64-apple-darwin")
        .clang_arg(format!("-L{}", target_header.display().to_string()))
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
}

#[cfg(all(target_os="linux", target_arch="x86_64"))]
fn generate_bindings(target_header: &Path) -> Result<Bindings,()> {
    builder()
        .header(target_header.display().to_string())
        .clang_arg("--target=x86_64-unknown-linux-musl")
        .clang_arg(format!("-L{}", target_header.display().to_string()))
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");

    println!("cargo:outdir={}", build_dir.display());
    std::fs::create_dir_all(&build_dir).expect("could not create build dir");

    // Initial build as JSCOnly;static;debug
    let result = Command::new("make")
        .args(&["-R", "-f", "makefile.cargo"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());

    // The input header we would like to generate bindings for.
    let cargo_manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_shim = cargo_manifest_dir
        .join("jsc-defs_cc153d7799a75e501fa5ca3f6a005744395cc32b")
        .join("JavaScriptCore.h");

    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
         let bindings = generate_bindings(&lib_shim.clone())
             .expect("Builder could not generate bindings D:");

         bindings
            .write_to_file(build_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        panic!("Current OS and/or Arch not supported. :(");
    }
}
