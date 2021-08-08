extern crate bindgen;
use ::std::env;
use ::std::path::PathBuf;
use ::std::process::{Command, Stdio};

fn main() {
    let makefile  = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("makefile.cargo");
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");

    // Initial build as JSCOnly;static;debug
    match Command::new("make")
        .args(&[
            "-R",
            "-f", makefile.to_str().expect("UTF-8"),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status() {
            // Make sure our compilation succeeded; else bail
            Ok(r) => assert!(r.success()),
            Err(e) => panic!("Make command failed, err: {:?}",e),
    }

    println!("cargo:rustc-link-search={}", build_dir.join("lib").display());
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");

    let bindings = bindgen::Builder::default()
        .header(build_dir.join("JavaScriptCore/PrivateHeaders/JavaScriptCore.h").to_str().expect("UTF-8"))
        .clang_args(&[
            "-L", build_dir.join("lib").to_str().expect("UTF-8"),
            "-l", ":libJavaScriptCore.a",
        ])
        .parse_callbacks(
            Box::new(bindgen::CargoCallbacks)
        )
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(build_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
