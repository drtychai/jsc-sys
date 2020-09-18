#![allow(unused_variables)]
extern crate jscjs_sys;
extern crate url;

use jscjs_sys::jsapi as api;
use jscjs_sys::{Context, Object, String, Value, VM};
use url::Url;

#[test]
fn simple() {
    unsafe {
        let vm = api::JSContextGroupCreate();
        api::JSContextGroupRelease(vm);
    }
}

#[test]
fn context() {
    let vm = VM::new();
    let context = Context::new(&vm);
    let string = String::new("Hello World");
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
        let source = Url::parse("https://webkit.org").unwrap();
        let object = Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("42", &object, source, 0).unwrap();
        assert!(result.is_number(&context));
    }
    {
        let source = Url::parse("https://webkit.org").unwrap();
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
        let source = Url::parse("https://webkit.org").unwrap();
        let result = context.check_syntax("function", source, 0);
        assert!(!result.is_ok());
    }
}
