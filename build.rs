extern crate bindgen;
use ::std::env;
use ::std::path::PathBuf;
use ::std::process::{Command, Stdio};

fn main() {
    let cargo_manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");

    // Condition upon rerun
    println!("cargo:rerun-if-changed={}/WebKit/Source/JavaScriptCore/API/JavaScript.h", cargo_manifest_dir.display());

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

    // Link our freshly built static libraries
    {
        println!("cargo:rustc-link-search=all={}/lib", cargo_manifest_dir.display());
        println!("cargo:rustc-link-lib=static=JavaScriptCore");
        println!("cargo:rustc-link-lib=static=WTF");
        println!("cargo:rustc-link-lib=static=bmalloc");
    }
        
    // We still need to link to system dylib for darwin builds
    {
        // By default, these are linked to:
        // path: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/lib
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=icucore");
        //println!("cargo:rustc-link-lib=-std=libc++");
    } 

    let mut builder = bindgen::builder()
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .header(cargo_manifest_dir
            .join("WebKit")
            .join("Source")
            .join("JavaScriptCore")
            .join("API")
            .join("JavaScript.h")
            .to_str().expect("UTF-8")
        )
        .clang_args(&[
            // libJavaScriptCore.a API run-time headers
            // path: WebKit/Source/JavaScriptCore/API
            "-I", cargo_manifest_dir
                .join("WebKit")
                .join("Source")
                .join("JavaScriptCore")
                .join("API").to_str().expect("UTF-8"),
            // libJavaScriptCore.a internal source headers
            // path: out/build/JavaScriptCore/PrivateHeaders
            "-I", build_dir
                .join("JavaScriptCore")
                .join("PrivateHeaders").to_str().expect("UTF-8"),
            // libWTF.a source headers 
            // path: WebKit/Source/WTF
            "-I", cargo_manifest_dir
                .join("WebKit")
                .join("Source")
                .join("WTF").to_str().expect("UTF-8"),
            // libbmalloc.a source headers
            // path: WebKit/Source/bmalloc
            "-I",
            cargo_manifest_dir
                .join("WebKit")
                .join("Source")
                .join("bmalloc").to_str().expect("UTF-8"),
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
    //"^kJS.*",
    // libWTF.a
    "WTF::.*",
    "_WTF::.*",
    // libicuuc.a libicui18n.a libicudata.a
    "icu::.*",
    "_u.*",
];

/// Functions we want to generate bindings to.
const ALLOWLIST_FUNCTIONS: &'static [&'static str] = &[
    // libJavaScriptCore.a
    "_JS.*",
    "JS[^C].*",
    // libWTF.a
    "WTF::.*",
    "_WTF::.*",
    // libicuuc.a libicui18n.a libicudata.a
    "icu::.*",
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
    ".*JSC::Intl.*",
    "root::CFStringRef",
];
