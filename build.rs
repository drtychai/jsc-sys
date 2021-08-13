extern crate bindgen;
//extern crate clang_sys;
//extern crate rust_icu;
use ::std::env;
use ::std::path::PathBuf;
use ::std::process::{Command, Stdio};

//use self::rust_icu::*;

fn main() {
    let cargo_manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    //let icu_lib_dir = PathBuf::from(env::var_os("ICU_LIB_PATH").unwrap());
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    println!("cargo:rerun-if-changed={}/WebKit/Source/JavaScriptCore/API/JavaScriptCore.h", cargo_manifest_dir.display());

    // Initial build as JSCOnly;static;debug
    match Command::new("make")
        .args(&[
            "-R",
            "-f", cargo_manifest_dir.join("makefile.cargo").to_str().expect("UTF-8"),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status() {
            // Make sure our compilation succeeded; else bail
            Ok(r) => assert!(r.success()),
            Err(e) => panic!("Make command failed, err: {:?}",e),
    }

    // Linker args
    {
        // Our (just) built [debug] static libraries
        println!("cargo:rustc-link-search=all={}/lib", build_dir.display());
        //println!("cargo:rustc-link-lib=framework=JavaScriptCore");
        //println!("cargo:rustc-link-lib=static=JavaScriptCore");
        //println!("cargo:rustc-link-lib=static=WTF");
        //println!("cargo:rustc-link-lib=static=bmalloc");
        println!("cargo:rustc-link-lib=c++");
        //println!("cargo:rustc-link-lib=-std=libc++");

        //// System local libicu-uc and libicu-i18n
        //// `brew install icu-uc icu-i18n`
        //println!("cargo:rustc-link-search=all={}", icu_lib_dir.display());
        ////println!("cargo:rustc-link-lib=static=icui18n");
        ////println!("cargo:rustc-link-lib=static=icuuc");
        ////println!("cargo:rustc-link-lib=static=icudata");
        //println!("cargo:rustc-link-lib=dylib=icui18n");
        //println!("cargo:rustc-link-lib=dylib=icuuc");
        //println!("cargo:rustc-link-lib=dylib=icudata");
    } 

    let mut builder = bindgen::builder()
        .rust_target(bindgen::RustTarget::Stable_1_40)
        .header(
            build_dir
                .join("JavaScriptCore")
                .join("PrivateHeaders")
                .join("JavaScriptCore.h").to_str().expect("UTF-8")
        )
        //.header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JavaScript.h").to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSBase.h").to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSContextRef.h").to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSStringRef.h").to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSObjectRef.h") .to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSTypedArray.h").to_str().expect("UTF-8"))
        .header(build_dir.join("JavaScriptCore").join("Headers").join("JavaScriptCore").join("JSValueRef.h").to_str().expect("UTF-8"))
        //
        //
        //.header(cargo_manifest_dir.join("WebKit").join("Source").join("WTF").join("icu").join("unicode").join("utf.h").to_str().expect("UTF-8"))
        //
        .clang_args(&[
            // API run-time headers
            "-I",
            cargo_manifest_dir
                .join("WebKit")
                .join("Source")
                .join("JavaScriptCore")
                .join("API").to_str().expect("UTF-8"),
            // Private include for internal functionality
            "-I",
            build_dir
                .join("JavaScriptCore")
                .join("PrivateHeaders").to_str().expect("UTF-8"),
            "-I",
            build_dir
                .join("JavaScriptCore")
                .join("PrivateHeaders")
                .join("JavaScriptCore").to_str().expect("UTF-8"),
            // libJavaScriptCore source headers 
            "-I",
            build_dir
                .join("JavaScriptCore")
                .join("DerivedSources").to_str().expect("UTF-8"),
            // libicu source headers 
            "-I",
            build_dir
                .join("ICU")
                .join("Headers")
                .join("unicode").to_str().expect("UTF-8"),
            // libWTF source headers 
            "-I",
            build_dir
                .join("WebKit")
                .join("Headers")
                .join("wtf").to_str().expect("UTF-8"),
        ])
        .enable_cxx_namespaces()
        // Translate every enum with the "rustified enum" strategy. We should
        // investigate switching to the "constified module" strategy, which has
        // similar ergonomics but avoids some potential Rust UB footguns.
        .rustified_enum(".*")
        // Translates csize_t to rust usize
        .size_t_is_usize(true);

    for ty in ALLOWLIST_TYPES {
        builder = builder.allowlist_type(ty);
    }

    for func in ALLOWLIST_FUNCTIONS {
        builder = builder.allowlist_function(func);
    }

    for item in BLOCKLIST_ITEMS {
        builder = builder.blocklist_item(item);
    }

    let bindings = builder
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(build_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Types which we want to generate bindings for (and every other type they
/// transitively use).
const ALLOWLIST_TYPES: &'static [&'static str] = &[
    // libJavaScriptCore.a
    "_JS.*",
    "JS[^C].*",
    // JSC API regex for enum exports of JSTypes
    "^kJS.*",
    // libWTF.a
    "WTF::.*",
    // libicuuc.a libicui18n.a libicudata.a
    "_u.*",
];

/// Functions we want to generate bindings to.
const ALLOWLIST_FUNCTIONS: &'static [&'static str] = &[
    // libJavaScriptCore.a
    "_JS.*",
    "JS[^C].*",
    // libWTF.a
    "WTF::.*",
    // libicuuc.a libicui18n.a libicudata.a
    "_u.*",
];

/// Types for which we should NEVER generate bindings, even if it is used within
/// a type or function signature that we are generating bindings for.
const BLOCKLIST_ITEMS: &'static [&'static str] = &[
    // Functions for which we should NEVER generate bindings to.
    "JSStringCreateWithCFString.*",
    "JSStringCopyCFString.*",

    // Types for which we should NEVER generate bindings, even if it is used within
    // a type or function signature that we are generating bindings for.
    "JSC::.*",
    "root::CFStringRef",
];
