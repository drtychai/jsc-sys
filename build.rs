extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
fn link_libjsc() {
    let cargo_manifest_dir = if let Some(env_var) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(env_var)
    } else {
        PathBuf::from(env::current_dir().unwrap())
    };

    //let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")).unwrap();
    let result = Command::new(cargo_manifest_dir.join("build__x86_64-unknown-linux-gnu.sh").display())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());

    println!("cargo:rustc-link-search={}", cargo_manifest_dir.join("WebKit/lib").display());
    println!("cargo:rustc-link-lib=static=bmalloc");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=gtest");
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
}


#[cfg(target_os = "macos")]
fn link_libjsc() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}



fn main() {
    // Ensure JavsScriptCore framework is included in bindgen context
    // This means either building JSC from source (linux) or using the
    // native JavaScriptCore.framework (macos)
    link_libjsc();

    // Generate FFI bindings using bindgen::Builder
    //
    // Bindings are created for the definitions
    // in wrapper.h and saved to ${OUT_DIR}/bindings.rs
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write generated FFI bindings to file
    // Thesee bindings are included as: jscjs_sys::runtime::api
    let bindings_rs = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(bindings_rs.display())
        .expect("Couldn't write bindings!");
}
