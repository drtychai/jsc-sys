/// Opaque type bound to a chunk of virtual memory. Owns at least one (JavaScript) execution enironment.
/// Struct of `JSContext` objects.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContextGroup([u8; 0]);
pub type JSContextGroupRef = *const OpaqueJSContextGroup;

/// Opaqe type bound to an execution context in a given VM (JSCContextGroup object)
/// Struct of global objects and execution state pointers.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContext([u8; 0]);
pub type JSContextRef = *const OpaqueJSContext;

/// A special JSContextRef type that's bound to the global execution context of JavaScriptCore
pub type JSGlobalContextRef = *mut OpaqueJSContext;

// Impl JSCContextGroupRef
extern "C" {
    pub fn JSContextGroupCreate() -> JSContextGroupRef;
}
extern "C" {
    pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
}
extern "C" {
    pub fn JSContextGroupRelease(group: JSContextGroupRef);
}
// Impl JSGlobalContextRef
extern "C" {
    pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;
}
extern "C" {
    pub fn JSGlobalContextCreateInGroup(
        group: JSContextGroupRef,
        globalObjectClass: JSClassRef,
    ) -> JSGlobalContextRef;
}
extern "C" {
    pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;
}
extern "C" {
    pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);
}
extern "C" {
    pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;
}
extern "C" {
    pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
}
// Impl JSContextRef
extern "C" {
    pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;
}
extern "C" {
    pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;
}
extern "C" {
    pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;
}


/// Opaque type bound to a UTF-16 character buffer.
/// The base representation of a String in JavaScriptCore.
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSString([u8; 0]);
pub type JSStringRef = *mut OpaqueJSString;

pub type JSChar = ::std::os::raw::c_ushort;
pub type size_t = ::std::os::raw::c_long;

// Impl JSStringRef
extern "C" {
    pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: size_t) -> JSStringRef;
}
extern "C" {
    pub fn JSStringCreateWithUTF8CString(string: *const ::std::os::raw::c_char) -> JSStringRef;
}
extern "C" {
    pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
}
extern "C" {
    pub fn JSStringRelease(string: JSStringRef);
}
extern "C" {
    pub fn JSStringGetLength(string: JSStringRef) -> size_t;
}
extern "C" {
    pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
}
extern "C" {
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> size_t;
}
extern "C" {
    pub fn JSStringGetUTF8CString(
        string: JSStringRef,
        buffer: *mut ::std::os::raw::c_char,
        bufferSize: size_t,
    ) -> size_t;
}


/// Opaque type bound to UTF-16 character buffer.
/// The base representation of a Class in JavaScriptCore.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSClass([u8; 0]);
pub type JSClassRef = *mut OpaqueJSClass;

/// Opaque type bound to UTF-16 character buffer.
/// A collection of JavaScript Object property names.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameArray([u8; 0]);
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameAccumulator([u8; 0]);
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;

pub type JSTypedArrayBytesDeallocator = ::std::option::Option<
    unsafe extern "C" fn(
        bytes: *mut ::std::os::raw::c_void,
        deallocatorContext: *mut ::std::os::raw::c_void,
    ),
>;

/// Opaque type bound to UTF-16 character buffer.
/// The base representation of all valus in JavaScriptCore.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSValue([u8; 0]);
pub type JSValueRef = *const OpaqueJSValue;

/// A special JSValueRef used as the base JavaScript Object
pub type JSObjectRef = *mut OpaqueJSValue;

#[no_mangle]
extern "C" {
    /// Evaluates a string of JavaScript.
    ///
    /// * `ctx`: The execution context to use.
    /// * `script`: A `JSString` containing the script to evaluate.
    /// * `thisObject`: The object to use as `this`, or `NULL` to
    ///   use the global object as `this`.
    /// * `sourceURL`: A `JSString` containing a URL for the script's
    ///   source file. This is used by debuggers and when reporting
    ///   exceptions. Pass `NULL` if you do not care to include source
    ///   file information.
    /// * `startingLineNumber`: An integer value specifying the script's
    ///   starting line number in the file located at `sourceURL`. This
    ///   is only used when reporting exceptions. The value is one-based,
    ///   so the first line is line `1` and invalid values are clamped
    ///   to `1`.
    /// * `exception`: A pointer to a `JSValueRef` in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// The `JSValue` that results from evaluating script, or `NULL` if an exception is thrown.
    pub fn JSEvaluateScript(
        ctx: JSContextRef,
        script: JSStringRef,
        thisObject: JSObjectRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Checks for syntax errors in a string of JavaScript.
    ///
    /// * `ctx`: The execution context to use.
    /// * `script`: A `JSString` containing the script to check for
    ///   syntax errors.
    /// * `sourceURL`: A `JSString` containing a URL for the script's
    ///   source file. This is only used when reporting exceptions.
    ///   Pass `NULL` if you do not care to include source file
    ///   information in exceptions.
    /// * `startingLineNumber`: An integer value specifying the script's
    ///   starting line number in the file located at `sourceURL`. This
    ///   is only used when reporting exceptions. The value is one-based,
    ///   so the first line is line `1` and invalid values are clamped
    ///   to `1`.
    /// * `exception`: A pointer to a `JSValueRef` in which to store a
    ///   syntax error exception, if any. Pass `NULL` if you do not care
    ///   to store a syntax error exception.
    ///
    /// Returns `true` if the script is syntactically correct, otherwise `false`.
    pub fn JSCheckScriptSyntax(
        ctx: JSContextRef,
        script: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> bool;
}
#[no_mangle]
extern "C" {
    /// Performs a JavaScript garbage collection.
    ///
    /// JavaScript values that are on the machine stack, in a register,
    /// protected by `JSValueProtect`, set as the global object of an
    /// execution context, or reachable from any such value will not
    /// be collected.
    ///
    /// During JavaScript execution, you are not required to call this
    /// function; the JavaScript engine will garbage collect as needed.
    /// JavaScript values created within a context group are automatically
    /// destroyed when the last reference to the context group is released.
    ///
    /// * `ctx`: The execution context to use.
    pub fn JSGarbageCollect(ctx: JSContextRef);
}

/// A constant identifying the type of a `JSValue`.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSType {
    /// The unique `undefined` value.
    Undefined = 0,
    /// The unique `null` value.
    Null = 1,
    /// A primitive boolean value; `true` or `false`.
    Boolean = 2,
    /// A primitive number value.
    Number = 3,
    /// A primitive string value.
    String = 4,
    /// An object value.
    Object = 5,
}

/// A constant identifying the Typed Array type of a `JSObjectRef`.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSTypedArrayType {
    /// `Int8Array`
    Int8Array = 0,
    /// `Int16Array`
    Int16Array = 1,
    /// `Int32Array`
    Int32Array = 2,
    /// `Uint8Array`
    Uint8Array = 3,
    /// `Uint8ClampedArray`
    Uint8ClampedArray = 4,
    /// `Uint16Array`
    Uint16Array = 5,
    /// `Uint32Array`
    Uint32Array = 6,
    /// `Float32Array`
    Float32Array = 7,
    /// `Float64Array`
    Float64Array = 8,
    /// `ArrayBuffer`
    ArrayBuffer = 9,
    /// Not a Typed Array
    None = 10,
}

#[no_mangle]
extern "C" {
    /// Returns a JavaScript value's type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` whose type you want to obtain.
    ///
    /// Returns a value of type `JSType` that identifies `value`'s type.
    pub fn JSValueGetType(ctx: JSContextRef, arg1: JSValueRef) -> JSType;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `undefined` type, otherwise `false`.
    pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `null` type, otherwise `false`.
    pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `boolean` type, otherwise `false`.
    pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `number` type, otherwise `false`.
    pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `string` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `string` type, otherwise `false`.
    pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value's type is the `object` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value`'s type is the `object` type, otherwise `false`.
    pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value is an `object` with a given class in its class chain.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    /// * `jsClass`: The `JSClass` to test against.
    ///
    /// Returns `true` if `value` is an `object` and has `jsClass` in its
    /// class chain, otherwise `false`.
    pub fn JSValueIsObjectOfClass(
        ctx: JSContextRef,
        value: JSValueRef,
        jsClass: JSClassRef,
    ) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value is an `array`.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value` is an `array`, otherwise `false`.
    pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value is a `date`.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    ///
    /// Returns `true` if `value` is a `date`, otherwise `false`.
    pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Returns a JavaScript value's Typed Array type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` whose Typed Array type to return.
    /// * `exception`: A pointer to a `JSValueRef` in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a value of type `JSTypedArrayType` that identifies
    /// value's Typed Array type, or `JSTypedArrayType::None` if the
    /// value is not a Typed Array object.
    pub fn JSValueGetTypedArrayType(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSTypedArrayType;
}
#[no_mangle]
extern "C" {
    /// Tests whether two JavaScript values are equal, as compared by the JS `==` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `a`: The first value to test.
    /// * `b`: The second value to test.
    /// * `exception`: A pointer to a `JSValueRef` in which to
    ///   store an exception, if any. Pass `NULL` if you do
    ///   not care to store an exception.
    ///
    /// Returns `true` if the two values are equal, `false` if
    /// they are not equal or an exception is thrown.
    pub fn JSValueIsEqual(
        ctx: JSContextRef,
        a: JSValueRef,
        b: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether two JavaScript values are strict equal, as compared
    /// by the JS `===` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `a`: The first value to test.
    /// * `b`: The second value to test.
    ///
    /// Returns `true` if the two values are strict equal, otherwise `false`.
    pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Tests whether a JavaScript value is an object constructed by a
    /// given constructor, as compared by the JS `instanceof` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to test.
    /// * `constructor`: The constructor to test against.
    /// * `exception`: A pointer to a `JSValueRef` in which to
    ///   store an exception, if any. Pass `NULL` if you do
    ///   not care to store an exception.
    ///
    /// Returns `true` if value is an object constructed by constructor,
    /// as compared by the JS `instanceof` operator, otherwise `false`.
    pub fn JSValueIsInstanceOfConstructor(
        ctx: JSContextRef,
        value: JSValueRef,
        constructor: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value of the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `undefined` value.
    pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value of the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `null` value.
    pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value of the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `boolean`: The `bool` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `boolean` type, representing the value of `boolean`.
    pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value of the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `number`: The `f64` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `number` type, representing the value of `number`.
    pub fn JSValueMakeNumber(ctx: JSContextRef, number: f64) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value of the string type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The `JSString` to assign to the newly created
    ///   `JSValue`. The newly created `JSValue` retains string, and
    ///   releases it upon garbage collection.
    ///
    /// Returns a `JSValue` of the `string` type, representing the value of `string`.
    pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript value from a JSON formatted string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The `JSString` containing the JSON string to be parsed.
    ///
    /// Returns a `JSValue` containing the parsed value, or `NULL` if the input is invalid.
    pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
#[no_mangle]
extern "C" {
    /// Creates a JavaScript string containing the JSON serialized representation of a JS value.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The value to serialize.
    /// * `indent`: The number of spaces to indent when nesting.
    ///   If `0`, the resulting JSON will not contains newlines.
    ///   The size of the indent is clamped to `10` spaces.
    /// * `exception`: A pointer to a `JSValueRef` in which to
    ///   store an exception, if any. Pass `NULL` if you do not
    ///   care to store an exception.
    ///
    /// Returns a `JSString` with the result of serialization, or `NULL` if an exception is thrown.
    pub fn JSValueCreateJSONString(
        ctx: JSContextRef,
        value: JSValueRef,
        indent: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
}
#[no_mangle]
extern "C" {
    /// Converts a JavaScript value to boolean and returns the resulting boolean.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to convert.
    ///
    /// Returns the boolean result of conversion.
    pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
#[no_mangle]
extern "C" {
    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to convert.
    /// * `exception`: A pointer to a `JSValueRef` in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// Returns the numeric result of conversion, or `NaN` if an exception is thrown.
    pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> f64;
}
#[no_mangle]
extern "C" {
    /// Converts a JavaScript value to string and copies the result into a JavaScript string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to convert.
    /// * `exception`:  A pointer to a `JSValueRef` in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// Returns a `JSString` with the result of conversion, or `NULL`
    /// if an exception is thrown. Ownership follows the Create Rule.
    pub fn JSValueToStringCopy(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
}
#[no_mangle]
extern "C" {
    /// Converts a JavaScript value to object and returns the resulting object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to convert.
    /// * `exception`: A pointer to a `JSValueRef` in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to store
    ///   an exception.
    ///
    /// Returns the `JSObject` result of conversion, or `NULL` if
    /// an exception is thrown.
    pub fn JSValueToObject(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
#[no_mangle]
extern "C" {
    /// Protects a JavaScript value from garbage collection.
    ///
    /// Use this method when you want to store a `JSValue` in a
    /// global or on the heap, where the garbage collector will
    /// not be able to discover your reference to it.
    ///
    /// A value may be protected multiple times and must be
    /// unprotected an equal number of times before becoming
    /// eligible for garbage collection.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to protect.
    pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);
}
#[no_mangle]
extern "C" {
    /// Unprotects a JavaScript value from garbage collection.
    ///
    /// A value may be protected multiple times and must be unprotected
    /// an equal number of times before becoming eligible for garbage
    /// collection.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The `JSValue` to unprotect.
    pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}

// Impl JSObjectRef
extern "C" {
    pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
extern "C" {
    pub fn JSObjectCallAsFunction(
        ctx: JSContextRef,
        object: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
extern "C" {
    pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
extern "C" {
    pub fn JSObjectMake(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        data: *mut ::std::os::raw::c_void,
    ) -> JSObjectRef;
}
extern "C" {
    pub fn JSObjectMakeArray(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
extern "C" {
    pub fn JSObjectMakeDate(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
extern "C" {
    pub fn JSObjectMakeError(
        ctx: JSContextRef,
        argumentCount: size_t,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
// Impl TypedArray
extern "C" {
    pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        byteOffset: size_t,
        length: size_t,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
extern "C" {
    pub fn JSObjectGetTypedArrayBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JSObjectGetTypedArrayLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
}
extern "C" {
    pub fn JSObjectGetTypedArrayByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
}
extern "C" {
    pub fn JSObjectGetTypedArrayByteOffset(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
}
extern "C" {
    pub fn JSObjectGetTypedArrayBuffer(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
// Impl ArrayBuffer
extern "C" {
    pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
        ctx: JSContextRef,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: size_t,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
extern "C" {
    pub fn JSObjectGetArrayBufferBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JSObjectGetArrayBufferByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> size_t;
}
