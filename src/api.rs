#[allow(deref_nullptr, dead_code, non_camel_case_types)]
mod prefix {
    include!(concat!(env!("OUT_DIR"),"/build/bindings.rs"));
}

pub use self::prefix::root::*;

extern "C" {
    pub fn JSContextGroupCreate() -> JSContextGroupRef;
    pub fn JSContextGroupRelease(raw_ctx: JSContextGroupRef);
    pub fn JSCheckScriptSyntax(
        raw_ctx: *const OpaqueJSContext,
        raw_str: JSStringRef,
        opaque_str: *mut OpaqueJSString,
        start_line: i32,
        exception_opaque_jsvalue: &mut *const OpaqueJSValue
    );
}
