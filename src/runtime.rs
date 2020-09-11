#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(warnings)]
#![allow(deprecated)]

extern crate url;
use libc::size_t;
use std::default::Default;
use std::ffi::CString;
use std::ptr;

use crate::jsapi::*;

pub struct VM {
    raw: JSContextGroupRef,
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            VM {
                raw: JSContextGroupCreate(),
            }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            JSContextGroupRelease(self.raw);
        }
    }
}

// JSC managed String.
pub struct String {
    raw: JSStringRef,
}

impl String {
    pub fn new(s: &str) -> String {
        let cstr = CString::new(s.as_bytes()).unwrap();
        unsafe {
            String {
                raw: JSStringCreateWithUTF8CString(cstr.as_ptr()),
            }
        }
    }

    pub fn length(&self) {
        unsafe {
            JSStringGetLength(self.raw);
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            JSStringRelease(self.raw);
        }
    }
}

pub struct Context {
    raw: JSGlobalContextRef,
}

impl Context {
    pub fn new(vm: &VM) -> Context {
        unsafe {
            Context {
                raw: JSGlobalContextCreateInGroup(vm.raw, ptr::null_mut()),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            JSGlobalContextRelease(self.raw);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Value {
    raw: JSValueRef,
}

pub type JSResult<T> = Result<T, Value>;

// Value is GC-managed. So it does not implement Drop trait.
impl Value {
    pub fn with_boolean(ctx: &Context, value: bool) -> Value {
        unsafe {
            Value {
                raw: JSValueMakeBoolean(ctx.raw, value),
            }
        }
    }

    pub fn with_number(ctx: &Context, value: f64) -> Value {
        unsafe {
            Value {
                raw: JSValueMakeNumber(ctx.raw, value),
            }
        }
    }

    pub fn with_string(ctx: &Context, value: &str) -> Value {
        unsafe {
            Value {
                raw: JSValueMakeString(ctx.raw, String::new(value).raw),
            }
        }
    }

    pub fn null(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: JSValueMakeNull(ctx.raw),
            }
        }
    }

    pub fn undefined(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: JSValueMakeUndefined(ctx.raw),
            }
        }
    }

    pub fn is_boolean(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsBoolean(ctx.raw, self.raw) }
    }

    pub fn is_null(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsNull(ctx.raw, self.raw) }
    }

    pub fn is_undefined(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsUndefined(ctx.raw, self.raw) }
    }

    pub fn is_number(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsNumber(ctx.raw, self.raw) }
    }

    pub fn is_string(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsString(ctx.raw, self.raw) }
    }

    pub fn is_object(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsObject(ctx.raw, self.raw) }
    }

    pub fn is_array(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsArray(ctx.raw, self.raw) }
    }

    pub fn is_date(&self, ctx: &Context) -> bool {
        unsafe { JSValueIsDate(ctx.raw, self.raw) }
    }

    pub fn is_empty(&self) -> bool {
        self.raw == ptr::null()
    }

    pub fn to_number(&self, ctx: &Context) -> JSResult<f64> {
        unsafe {
            let mut exception: JSValueRef = ptr::null_mut();
            let result = JSValueToNumber(ctx.raw, self.raw, &mut exception);
            if exception == ptr::null() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn to_boolean(&self, ctx: &Context) -> bool {
        unsafe { JSValueToBoolean(ctx.raw, self.raw) }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value { raw: ptr::null() }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Object {
    raw: JSObjectRef,
}

impl Object {
    pub fn array(ctx: &Context, arguments: &[Value]) -> JSResult<Object> {
        unsafe {
            let mut exception: JSValueRef = ptr::null_mut();
            let result = JSObjectMakeArray(
                ctx.raw,
                arguments.len() as size_t,
                arguments.as_ptr() as *mut JSValueRef,
                &mut exception,
            );
            if exception == ptr::null_mut() {
                Ok(Object { raw: result })
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn is_constructor(&self, ctx: &Context) -> bool {
        unsafe { JSObjectIsConstructor(ctx.raw, self.raw) }
    }
}

impl Default for Object {
    fn default() -> Object {
        Object {
            raw: ptr::null_mut(),
        }
    }
}

impl Context {
    pub fn evaluate_script(&self, script: &str, receiver: &Object, url: url::Url, starting_line_number: i32) -> JSResult<Value> {
        let string = String::new(script);
        let source = String::new(url.as_str());
        unsafe {
            let mut exception: JSValueRef = ptr::null_mut();
            let result = JSEvaluateScript(
                self.raw,
                string.raw,
                receiver.raw,
                source.raw,
                starting_line_number,
                &mut exception,
            );
            if exception == ptr::null_mut() {
                Ok(Value { raw: result })
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn check_syntax(&self, script: &str, url: url::Url, starting_line_number: i32) -> JSResult<bool> {
        let string = String::new(script);
        let source = String::new(url.as_str());
        unsafe {
            let mut exception: JSValueRef = ptr::null_mut();
            let result = JSCheckScriptSyntax(
                self.raw,
                string.raw,
                source.raw,
                starting_line_number,
                &mut exception,
            );
            if exception == ptr::null_mut() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }
}
