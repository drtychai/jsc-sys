#![crate_name = "jscjs_sys"]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, improper_ctypes)]
//!
//! This crate contains Rust bindings to the Webkit JavaScript engine, [JavaScriptCore][1],
//! developed by Apple.
//!
//! These bindings are designed to be a fairly straightforward translation to the low-level C API,
//! while taking advantage of Rust's memory safety. For more about the JavaScriptCore API, see the
//! API [source][2] and the [documentation][3].
//!
//! Provided below are some practical examples of what functionalities this crate allows:
//!   - Create a global scripting context, used to create and execute JavaScript objects and code
//!   - Work natively with objects, parameters
//!   - Build JavaScript functions out of strings
//!   - Associate C callbacks to user-definted "classes" of objects
//!   - Attach C callbacks to "classes", handles responses to an action (e.g., getters/setters, promises, fn cals)
//!   - Load JavaScript files based on designated names and starting line-numbers
//!
//! [1]: https://trac.webkit.org/wiki/JavaScriptCore
//! [2]: https://github.com/WebKit/webkit/tree/master/Source/JavaScriptCore/API
//! [3]: https://developer.apple.com/documentation/javascriptcore
//!

pub mod runtime;

pub use self::runtime::api;
pub use self::runtime::{Context, Object, String, Value, VM};
