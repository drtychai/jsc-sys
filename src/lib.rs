#![crate_name = "jscjs_sys"]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, improper_ctypes, unused_imports)]

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

use self::runtime::api;
pub use self::runtime::{Context, Object, String, Value, VM};

#[test]
fn simple() {
    unsafe {
        let vm = runtime::api::JSContextGroupCreate();
        runtime::api::JSContextGroupRelease(vm);
    }
}

#[test]
fn context() {
    let vm = VM::new();
    let context = Context::new(&vm);
    let _string = String::new("Hello World");
    {
        let value = Value::with_boolean(&context, false);
        assert!(value.is_boolean(&context));
    }

    {
        let value = Value::with_number(&context, 42 as f64);
        match value.to_number(&context) {
            Ok(n) => assert_eq!(n, 42 as f64),
            Err(_) => unreachable!(),
        }
    }
}

#[test]
fn eval() {
    let vm = VM::new();
    let context = Context::new(&vm);
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("42", &object, source, 0).unwrap();
        assert!(result.is_number(&context));
    }
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("deadbeef", &object, source, 0);
        assert!(!result.is_ok());
    }
}

#[test]
fn check_syntax() {
    let vm = VM::new();
    let context = Context::new(&vm);

    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let result = context.check_syntax("function", source, 0);
        assert!(!result.is_ok());
    }
}
