#![no_builtins]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    deprecated,
)]

extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    println!("cargo:outdir={}", build_dir.display());

    build_jsapi(&build_dir);
    // if we need to redine/inline fn's to replace the mac-specific ones, it will be as this is done
    //build_jsglue(&build_dir);
    build_jsapi_bindings(&build_dir);

    println!(
        "cargo:rerun-if-env-changed={}",
        build_dir.join("bindings.rs").to_str().expect("UTF-8")
    );
}

fn cc_flags(build_dir: &Path) -> Vec<&'static str> {
    let mut clang_lib_arg = String::from("-L").to_owned();
    clang_lib_arg.push_str(build_dir.join("lib").to_str().expect("UTF-8"));

    let mut result = vec![
        "-DRUST_BINDGEN",
        "-DENABLE_STATIC_JSC=ON",
        "-DCMAKE_C_COMPILER=/usr/local/bin/clang",
        "-DCMAKE_CXX_COMPILER=/usr/local/bin/clang++",
        "-DCMAKE_CXX_FLAGS='-O3 -lrt'",
    ];

    result.extend(&[
        //"-j 6",
        //"--max-load 8",
        //"-Wsuggest-override",
        "-Wall -Werror -Wunused-but-set-variable -ftree-slp-vectorize",
        // GLib headers
        //"-I/usr/include/glib-2.0 -I/usr/lib/x86_64-linux-gnu/glib-2.0/include",

        "-fno-sized-deallocation",
        "-Wno-unused-parameter",
        "-Wno-invalid-offsetof",
        "-Wno-unused-private-field",
    ]);

    result
}

fn build_jsapi(build_dir: &Path) {
    let makefile = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("makefile.cargo");
    println!("makefile: {}", makefile.display());

    // Initial build as JSCOnly;static;debug
    let result = Command::new("make")
        .args(&["-R", "-f", makefile.to_str().expect("UTF-8")])
        .current_dir(build_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());

    println!("cargo:rustc-link-=static=search={}", build_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=bmalloc");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
}

/// Invoke bindgen on the JSAPI headers to produce raw FFI bindings for use from
/// Rust.
///
/// To add or remove which functions, types, and variables get bindings
/// generated, see the `const` configuration variables below.
fn build_jsapi_bindings(build_dir: &Path) {
    let jsc_include_dir = build_dir
        .join("DerivedSources")
        .join("ForwardingHeaders")
        .join("JavaScriptCore")
        .join("JavaScriptCore.h");

    let wrapper_h = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("wrapper.h");
    //let wrapper_h = jsc_include_dir.join("JavaScriptCore").join("JavaScriptCore.h");

    let mut config = bindgen::CodegenConfig::all();
    config &= !bindgen::CodegenConfig::CONSTRUCTORS;
    config &= !bindgen::CodegenConfig::DESTRUCTORS;
    config &= !bindgen::CodegenConfig::VARS;
    config &= !bindgen::CodegenConfig::METHODS;
    config &= !bindgen::CodegenConfig::TYPES;
    config &= !bindgen::CodegenConfig::FUNCTIONS;

    let mut builder = bindgen::Builder::default()
        .use_core()
        .rust_target(bindgen::RustTarget::Stable_1_40)
        .header(wrapper_h.to_str().expect("UTF-8"))
        // Translate every enum with the "rustified enum" strategy
        .rustified_enum(".*")
        .size_t_is_usize(true)
        .enable_cxx_namespaces()
        .with_codegen_config(config)
        .rustfmt_bindings(true);

    // Specify our build architecture and target headers to generate bindings for
    builder = builder
        .clang_arg("-I")
        .clang_arg(jsc_include_dir.to_str().expect("UTF-8"))
        .clang_arg("-include")
        .clang_arg(wrapper_h.to_str().expect("UTF-8"));

    for js_type in WHITELIST_TYPES {
        builder = builder.whitelist_type(js_type);
    }

    for js_var in WHITELIST_VARS {
        builder = builder.whitelist_var(js_var);
    }

    for js_func in WHITELIST_FUNCTIONS {
        builder = builder.whitelist_function(js_func);
    }

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

/// Types which we want to generate bindings for (and every other type they
/// transitively use).
const WHITELIST_TYPES: &'static [&'static str] = &[
    //"JS.*",
    "JSC::.*",
    "JSGlobalContextRef",
    "JSContextGroupRef",
    "JSObjectRef",
    "JSStringRef",
    "JSValueRef",
    //"bmalloc::.*",
    //"WTF::.*",
    //"Inspector::.*",
];

/// Global variables we want to generate bindings to.
const WHITELIST_VARS: &'static [&'static str] = &[
    //"JS_.*",
];

/// Functions we want to generate bindings to.
const WHITELIST_FUNCTIONS: &'static [&'static str] = &[
    //"JS.*",
    //"jsc_.*:",
    "JSStringGetLength",
    "JSStringRelease",
    "JSStringCreateWithUTF8CString",

    "JSContextGroupRelease",
    "JSContextGroupCreate",

    "JSGlobalContextRelease",
    "JSGlobalContextCreateInGroup",
    "JSGlobalContextCreateRelease",

    "JSValueMakeBoolean",
    "JSValueMakeNumber",
    "JSValueMakeString",
    "JSValueMakeNull",
    "JSValueMakeUndefined",

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

    "JSObjectMakeArray",
    "JSObjectIsConstructor",
    "JSEvaluateScript",
    "JSCheckScriptSyntax",
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
