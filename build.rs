#![no_builtins]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    deprecated,
    dead_code
)]

extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    println!("cargo:outdir={}", build_dir.display());

    build_jscapi(&build_dir);
    build_jscapi_bindings(&build_dir);
}

fn build_jscapi(build_dir: &Path) {
    let makefile = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("makefile.cargo");

    let result = Command::new("make")
        .args(&["-R", "-f", makefile.to_str().expect("UTF-8")])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to run `make`");
    assert!(result.success());


    // Import newly build libJavaScriptCore[gtk-4.0]
    println!("cargo:rustc-link-search=native={}",build_dir.join("lib").display());
    {
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-lib=javascriptcoregtk-4.0");
            println!("cargo:rustc-link-lib=stdc++");
        }

        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-search={}", build_dir.join("lib").display());
            //println!("cargo:rustc-link-lib=static=libJavaScriptCore.a");
            println!("cargo:rustc-link-lib=framework=JavaScriptCore");
        }
    }
}

/// Invoke bindgen on the JSAPI headers to produce raw FFI bindings for use from
/// Rust.
///
/// To add or remove which functions, types, and variables get bindings
/// generated, see the `const` configuration variables below.
fn build_jscapi_bindings(build_dir: &Path) {
    let mut config = bindgen::CodegenConfig::all();
    config &= !bindgen::CodegenConfig::CONSTRUCTORS;
    config &= !bindgen::CodegenConfig::DESTRUCTORS;
    config &= !bindgen::CodegenConfig::VARS;
    config &= !bindgen::CodegenConfig::METHODS;
    config &= !bindgen::CodegenConfig::TYPES;
    config &= !bindgen::CodegenConfig::FUNCTIONS;

    let cargo_manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let mut builder = bindgen::Builder::default()
        .use_core()
        .rust_target(bindgen::RustTarget::Stable_1_40)
        // Translate every enum with the "rustified enum" strategy
        .rustified_enum(".*")
        .size_t_is_usize(true)
        .enable_cxx_namespaces()
        .with_codegen_config(config)
        .rustfmt_bindings(true);


    if cfg!(target_os = "linux") {
        builder = builder
            .header(cargo_manifest_dir
                    .join("WebKit/Source/JavaScriptCore/API/glib/jsc.h")
                    .to_str()
                    .expect("UTF-8"),
            )
            // JSC GTK headers
            // Provides builder with incude for `#include <jsc/[.*].h>`
            .clang_args(&[
                "-I", build_dir
                        .join("DerivedSources/ForwardingHeaders/JavaScriptCore/glib")
                        .to_str()
                        .expect("UTF-8"),
                "-I", build_dir
                        .join("DerivedSources/JavaScriptCore/javascriptcoregtk")
                        .to_str()
                        .expect("UTF-8"),
                "-include", cargo_manifest_dir
                        .join("WebKit/Source/JavaScriptCore/API/glib/jsc.h")
                        .to_str()
                        .expect("UTF-8")
            ])
            // GLib headers
            .clang_args(&[
                "-I",
                "/usr/include/glib-2.0",
                "-I",
                "/usr/lib/x86_64-linux-gnu/glib-2.0/include",
            ]);

    } else if cfg!(target_os = "macos") {
        builder = builder
            .header(build_dir
                .join("DerivedSources/ForwardingHeaders/JavaScriptCore/JavaScript.h")
                .to_str()
                .expect("UTF-8"));
    } else {
        panic!("Support is only available for x86_64-apple-darwin and x86_64-unknown-linux-gnu");
    }

    for js_type in WHITELIST_TYPES {
        builder = builder.whitelist_var(js_type);
    }

    for js_var in WHITELIST_VARS {
        builder = builder.whitelist_var(js_var);
    }

    for js_func in WHITELIST_FUNCTIONS {
        builder = builder.whitelist_function(js_func);
    }

    println!(
        "Generting bindings {:?} {}.",
        builder.command_line_flags(),
        bindgen::clang_version().full
    );
    let bindings = builder
        // If any header files in our target source are modified, re-run
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("An error occurred while generating JSC API bindings");

    let bindings_rs = build_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_rs)
        .expect("An error occurred while writing JSC API bindings to file");
}

fn cc_flags() -> Vec<&'static str> {
    let mut result = vec![
        "-D",
        "RUST_BINDGEN",
        "-D",
        "ENABLE_STATIC_JSC=ON",
        "-D",
        "JSC_GLIB_API_ENABLED=ON",
    ];

    result.extend(&[
        "-Wall -Werror",
        "-Wunused-but-set-variable",
        "-Wno-unused-parameter",
        "-Wno-invalid-offsetof",
        "-Wno-unused-private-field",
        "-ftree-slp-vectorize",
        "-fno-sized-deallocation",
    ]);

    result
}

/// Types which we want to generate bindings for (and every other type they
/// transitively use).
const WHITELIST_TYPES: &'static [&'static str] = &[
    // GLib types
    //"JSC::.*",
    "JSCValue",
    "JSCContext",
    "JSGlobalContextRef",
    "JSValueRef",
    "JSStringRef",
    // Darwin types
    //"JS.*",
    //"JSGlobalContextRef",
    //"JSContextGroupRef",
    //"JSObjectRef",
    //"JSStringRef",
    //"JSValueRef",
];

/// Global variables we want to generate bindings to.
const WHITELIST_VARS: &'static [&'static str] = &[
    //"JS_.*",
];

/// Functions we want to generate bindings to.
const WHITELIST_FUNCTIONS: &'static [&'static str] = &[
    //"JS.*",
    //"jsc_.*:",
    "JSValueIsBoolean",
    "JSValueIsNull",
    "JSValueIsUndefined",
    "JSValueIsNumber",
    "JSValueIsString",
    "JSValueIsObject",
    "JSValueIsArray",
    "JSValueIsDate",
    "JSValueToNumber",
    "JSValueToBoolean",
    "JSValueToStringCopy",
    "JSStringRelease",
    "JSStringGetMaximumUTF8CStringSize",
    "JSStringGetUTF8CString",
    //"JSStringGetLength",
    //"JSStringRelease",
    //"JSStringCreateWithUTF8CString",

    //"JSContextGroupRelease",
    //"JSContextGroupCreate",

    //"JSGlobalContextRelease",
    //"JSGlobalContextCreateInGroup",
    //"JSGlobalContextCreateRelease",

    //"JSObjectMakeArray",
    //"JSObjectIsConstructor",
    //"JSEvaluateScript",
    //"JSCheckScriptSyntax",
];

/// Types for which we should NEVER generate bindings, even if it is used within
/// a type or function signature that we are generating bindings for.
const _BLACKLIST_TYPES: &'static [&'static str] = &[
    // We'll be using libc::FILE.
    "FILE",
    // We provide our own definition because we need to express trait bounds in
    // the definition of the struct to make our Drop implementation correct.
    "JS::Heap",
    // We provide our own definition because SM's use of templates
    // is more than bindgen can cope with.
    "JS::Rooted",
    // We don't need them and bindgen doesn't like them.
    "JS::HandleVector",
    "JS::MutableHandleVector",
    "JS::Rooted.*Vector",
];
