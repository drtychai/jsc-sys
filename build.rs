extern crate bindgen;
use ::std::{env, fmt};
use ::std::path::PathBuf;
use ::std::process::{Command, Stdio};


fn build_jsc(cargo_manifest_dir: &PathBuf) -> self::fmt::Result {
    // Initial build as JSCOnly;static;debug
    match Command::new("make")
        .args(&[
            "-R",
            "-f",
            cargo_manifest_dir.join("makefile.cargo").to_str().expect("UTF-8"),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status() {
            // Make sure our compilation succeeded; else bail
            Ok(r) => assert!(r.success()),
            Err(e) => panic!("Make command failed, err: {:?}",e),
    }
    Ok(())
}

fn generate_bindings(build_dir: &PathBuf, cargo_manifest_dir: &PathBuf) -> self::fmt::Result {
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
            // libJavaScriptCore.a internal source headers
            // path: out/build/JavaScriptCore/PrivateHeaders
            "-I", build_dir
                .join("JavaScriptCore")
                .join("PrivateHeaders").to_str().expect("UTF-8"),
            // Only include public headers for non-darwin builds
            #[cfg(target_os = "linux")]
            "-I",
            #[cfg(target_os = "linux")]
            build_dir
                .join("JavaScriptCore")
                .join("Headers").to_str().expect("UTF-8"),
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(build_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    Ok(())
}

fn main() {
    let cargo_manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");

    // Bail if JSC build fails
    assert_eq!(
        build_jsc(&cargo_manifest_dir),
        Ok(())
    );

    // Link our freshly built static libraries
    // Applicable to both darwin and gnu
    println!("cargo:rustc-link-search=all={}/lib", cargo_manifest_dir.display());
    println!("cargo:rustc-link-lib=static=JavaScriptCore");
    println!("cargo:rustc-link-lib=static=WTF");
    println!("cargo:rustc-link-lib=static=bmalloc");
      
    // OS-specific linking
    #[cfg(target_os = "macos")]
    {
        // By default, these are linked to:
        println!("cargo:rustc-link-lib=icucore");
        println!("cargo:rustc-link-lib=c++");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=icui18n");
        println!("cargo:rustc-link-lib=icuuc");
        println!("cargo:rustc-link-lib=icudata");
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Bail if bindgen fails
    assert_eq!(
        generate_bindings(&build_dir, &cargo_manifest_dir),
        Ok(())
    );

}

/// Types which we want to generate bindings for (and every other type they
/// transitively use).
const ALLOWLIST_TYPES: &'static [&'static str] = &[
    // A group that associates JavaScript execution contexts with one another.
    "JSContextGroupRef",
    
    // A JavaScript execution context.
    "JSContextRef",
    
    // A global JavaScript execution context.
    "JSGlobalContextRef",
    
    // A UTF-16 character buffer.
    "JSStringRef",
    
    // A JavaScript class.
    "JSClassRef",
    
    // A JavaScript value.
    "JSValueRef",
    
    // A JavaScript object.
    "JSObjectRef",
];

/// Functions we want to generate bindings to.
const ALLOWLIST_FUNCTIONS: &'static [&'static str] = &[
    // Script Evaluation functions
     
    /// Checks for syntax errors in a string of JavaScript.
    // JSCheckScriptSyntax(
    //     JSContextRef!,
    //     JSStringRef!, 
    //     JSStringRef!, 
    //     Int32,
    //     UnsafeMutablePointer<JSValueRef?>!
    // ) -> Bool
    "JSCheckScriptSyntax",

    /// Evaluates a string of JavaScript.
    // JSEvaluateScript(
    //     JSContextRef!, 
    //     JSStringRef!, 
    //     JSObjectRef!, 
    //     JSStringRef!, 
    //     Int32, 
    //     UnsafeMutablePointer<JSValueRef?>!
    // ) -> JSValueRef!
    "JSEvaluateScript",

    /// Performs a JavaScript garbage collection.
    // "JSGarbageCollect(JSContextRef!)",
    "JSGarbageCollect",
 
    // Impls for allowlisted types
    "JSContextGroup.*",
    "JSContext.*",
    "JSGlobalContext.*",
    "JSString.*",
    "JSClass.*",
    "JSValue.*",
    "JSObject.*",
];

/// Types for which we should NEVER generate bindings, even if it is used within
/// a type or function signature that we are generating bindings for.
const BLOCKLIST_ITEMS: &'static [&'static str] = &[
    //// Functions for which we should NEVER generate bindings to.
    //"JSStringCreateWithCFString.*",
    //"JSStringCopyCFString.*",

    //// Types for which we should NEVER generate bindings, even if it is used within
    //// a type or function signature that we are generating bindings for.
    //"JSC::.*",
    //".*JSC::Intl.*",
    //"root::CFStringRef",
];


