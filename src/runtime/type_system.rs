use crate::api;
use ::std::{ffi, ptr};
use ::std::default::Default;

/// Local Execution Context
pub struct VM {
    raw: api::JSContextGroupRef,
}

impl VM {
    pub fn new() -> VM {
        unsafe { VM { raw: api::JSContextGroupCreate() } }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            api::JSContextGroupRelease(self.raw);
        }
    }
}

/// JSC managed String.
pub struct String {
    raw: api::JSStringRef,
}

impl String {
    pub fn new(s: &str) -> String {
        let cstr = ffi::CString::new(s.as_bytes()).unwrap();
        unsafe {
            String {
                raw: api::JSStringCreateWithUTF8CString(cstr.as_ptr()),
            }
        }
    }

    pub fn length(&self) {
        unsafe {
            api::JSStringGetLength(self.raw);
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            api::JSStringRelease(self.raw);
        }
    }
}

/// Global Execution Context
pub struct Context {
    raw: api::JSGlobalContextRef,
}

impl Context {
    pub fn new(vm: &VM) -> Context {
        unsafe {
            Context {
                raw: api::JSGlobalContextCreateInGroup(vm.raw, ptr::null_mut()),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            api::JSGlobalContextRelease(self.raw);
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// Wrapper around JSC types, e.g., Boolean, Number, etc.
pub struct Value {
    raw: api::JSValueRef,
}

pub type JSResult<T> = Result<T, Value>;

// Value is GC-managed. So it does not implement Drop trait.
impl Value {
    pub fn with_boolean(ctx: &Context, value: bool) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeBoolean(ctx.raw, value),
            }
        }
    }

    pub fn with_number(ctx: &Context, value: f64) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeNumber(ctx.raw, value),
            }
        }
    }

    pub fn with_string(ctx: &Context, value: &str) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeString(ctx.raw, String::new(value).raw),
            }
        }
    }

    pub fn null(ctx: &Context) -> Value {
        unsafe { Value { raw: api::JSValueMakeNull(ctx.raw) } }
    }

    pub fn undefined(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeUndefined(ctx.raw),
            }
        }
    }

    pub fn is_boolean(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsBoolean(ctx.raw, self.raw) }
    }

    pub fn is_null(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsNull(ctx.raw, self.raw) }
    }

    pub fn is_undefined(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsUndefined(ctx.raw, self.raw) }
    }

    pub fn is_number(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsNumber(ctx.raw, self.raw) }
    }

    pub fn is_string(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsString(ctx.raw, self.raw) }
    }

    pub fn is_object(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsObject(ctx.raw, self.raw) }
    }

    pub fn is_array(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsArray(ctx.raw, self.raw) }
    }

    pub fn is_date(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueIsDate(ctx.raw, self.raw) }
    }

    pub fn is_empty(&self) -> bool {
        self.raw == ptr::null()
    }

    pub fn to_number(&self, ctx: &Context) -> JSResult<f64> {
        unsafe {
            let mut exception: api::JSValueRef = ptr::null_mut();
            let result = api::JSValueToNumber(ctx.raw, self.raw, &mut exception);
            if exception == ptr::null() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn to_boolean(&self, ctx: &Context) -> bool {
        unsafe { api::JSValueToBoolean(ctx.raw, self.raw) }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value { raw: ptr::null() }
    }
}

#[derive(Copy, Clone, Debug)]
/// Top level structure for JavaScript. Equivalent to a HashMap.
pub struct Object {
    raw: api::JSObjectRef,
}

impl Object {
    pub fn array(ctx: &Context, arguments: &[Value]) -> JSResult<Object> {
        unsafe {
            let mut exception: api::JSValueRef = ptr::null_mut();
            let result = api::JSObjectMakeArray(
                ctx.raw,
                arguments.len() as usize,
                arguments.as_ptr() as *mut api::JSValueRef,
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
        unsafe { api::JSObjectIsConstructor(ctx.raw, self.raw) }
    }
}

impl Default for Object {
    fn default() -> Object {
        Object { raw: ptr::null_mut() }
    }
}

impl Context {
    pub fn evaluate_script(&self, script: &str, receiver: &Object, url: url::Url, starting_line_number: i32) -> JSResult<Value> {
        let string = String::new(script);
        let source = String::new(url.as_str());
        unsafe {
            let mut exception: api::JSValueRef = ptr::null_mut();
            let result = api::JSEvaluateScript(self.raw, string.raw, receiver.raw, source.raw, starting_line_number, &mut exception);
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
            let mut exception: api::JSValueRef = ptr::null_mut();
            let result = api::JSCheckScriptSyntax(self.raw, string.raw, source.raw, starting_line_number, &mut exception);
            if exception == ptr::null_mut() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::api;

    // If this test fails, we have an issue with out bindings/FFIs
    #[test]
    #[ignore]
    fn vm_context_ffi_smoke() {
        unsafe {
            let vm_raw = api::JSContextGroupCreate();
            assert_eq!(api::JSContextGroupRelease(vm_raw), ());
        }
    }
}
