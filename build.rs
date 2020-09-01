extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn make_bundled() {
    // Build JSC
    let cwd = env::current_dir().unwrap();
    let webkit_src_root = cwd.join("WebKit")
                             .join("WebKitBuild")
                             .join("Debug")
                             .join("lib");

    let result = Command::new("make")
                         .args(&["-R", "-f", "makefile.cargo"])
                         .stdout(Stdio::inherit())
                         .stderr(Stdio::inherit())
                         .status()
                         .unwrap();
    assert!(result.success());

    // Shim path bound independently, twice
    let jsc_header_shim = cwd.join("WebKit")
                             .join("WebKitBuild")
                             .join("Debug")
                             .join("DerivedSources")
                             .join("ForwardingHeaders")
                             .join("JavaScriptCore")
                             .join("JavaScript.h")
                             .into_os_string()
                             .into_string()
                             .unwrap();

    let clang_shim = format!("-I{}", cwd.join("WebKit")
                              .join("WebKitBuild")
                              .join("Debug")
                              .join("DerivedSources")
                              .join("ForwardingHeaders")
                              .join("JavaScriptCore")
                              .join("JavaScript.h")
                              .display());

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(jsc_header_shim)
        .clang_arg(clang_shim)
        .clang_arg("--target=x86_64-apple-darwin")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
       .write_to_file(out_path.join("bindings.rs"))
       .expect("Couldn't write bindings!");


    println!("cargo:rustc-link-search=native={}", webkit_src_root.display());
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=bmalloc");

    if cfg!(not(target_os = "macos")) {
        {
            let lib = pkg_config::find_library("icu-uc").unwrap();
            for library in &lib.libs {
                println!("cargo:rustc-link-lib=dylib={}", library);
            }
        }

        {
            let lib = pkg_config::find_library("icu-i18n").unwrap();
            for library in &lib.libs {
                println!("cargo:rustc-link-lib=dylib={}", library);
            }
        }

        println!("cargo:rustc-link-lib=stdc++");
    } else {
        //println!("cargo:rustc-link-lib=icucore");
        println!("cargo:rustc-link-lib=c++");
    }
}

fn make_system_darwin() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let is_macos = target_os == "macos";
    let is_intel = target_arch == "x86" || target_arch == "x86_64";

    println!("macos: {}, intel: {}", is_macos, is_intel);
    println!("target: {}", env::var("TARGET").unwrap());

    let sysroot = if is_macos {
        "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk"
    } else {
        unreachable!()
    };

    let bindings = bindgen::Builder::default()
        .header(format!("{}/System/Library/Frameworks/JavaScriptCore.framework/Headers/JavaScript.h", sysroot))
        .clang_arg("--target=x86_64-apple-darwin") // Needed to stop libclang crashing with _NO_ error.
        .clang_arg("-isysroot")
        .clang_arg(sysroot)
        .clang_arg("-iframework")
        .clang_arg("JavaScriptCore")
        .clang_arg("-U__APPLE__")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    if cfg!(target_os = "linux") {
        make_bundled()
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        make_system_darwin()
    } else {
        panic!("Unsupported target architecture.")
    }
}
