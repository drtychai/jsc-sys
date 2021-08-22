/// Rust FFI bindings to JavaScriptCore's CXX runtime API
#[cxx::bridge]
mod ffi {
    // Shared structs with fields visible to both languages.
    {
        // --- 
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        //include!("WebKit/Source/JavaScriptCore/API/JavaScript.h");

        //// Structs
        type JSClassDefinition;
        type JSStaticFunction;
        type JSStaticValue;
        type OpaqueJSClass;
        type OpaqueJSContext;
        type OpaqueJSContextGroup;
        type OpaqueJSPropertyNameAccumulator;
        type OpaqueJSPropertyNameArray;
        type OpaqueJSString;
        type OpaqueJSValue;
       
        //// Enums
        type JSType;
        type JSTypedArrayType;

        //// Typedefs
        type JSChar;
        type JSClassAttributes;
        
        /// A JavaScript class.
        type JSClassRef;
        
        /// A group that associates JavaScript execution contexts with one another.
        type JSContextGroupRef;
        
        /// A JavaScript execution context.
        type JSContextRef;
        
        /// A global JavaScript execution context.
        type JSGlobalContextRef;
        type JSObjectCallAsConstructorCallback;
        type JSObjectCallAsFunctionCallback;
        type JSObjectConvertToTypeCallback;
        type JSObjectDeletePropertyCallback;
        type JSObjectFinalizeCallback;
        type JSObjectGetPropertyCallback;
        type JSObjectGetPropertyNamesCallback;
        type JSObjectHasInstanceCallback;
        type JSObjectHasPropertyCallback;
        type JSObjectInitializeCallback;
        
        /// A JavaScript object.
        type JSObjectRef;
        type JSObjectSetPropertyCallback;
        type JSPropertyAttributes;
        type JSPropertyNameAccumulatorRef;
        type JSPropertyNameArrayRef;
        
        /// A UTF-16 character buffer.
        type JSStringRef;
        type JSTypedArrayBytesDeallocator;
        
        /// A JavaScript value.
        type JSValueRef;


        //// Script Functions
        /// Checks for syntax errors in a string of JavaScript.
        fn JSCheckScriptSyntax(
                ctx: JSContextRef, 
                script: JSStringRef, 
                sourceURL: JSStringRef, 
                startingLineNumber: i32, 
                exception: &mut JSValueRef,
        ) -> bool;

        /// Evaluates a string of JavaScript.        
        fn JSEvaluateScript(
            // The execution context to use.
            ctx: JSContextRef, 
            // Contains the script to evaluate.
            script: JSStringRef, 
            // The object to use as this or NULL to use the global object as this.
            thisObject: JSObjectRef, 
            // A JSStringRef that contains a URL for the script's source file. 
            // The system only uses this when reporting exceptions.
            // Pass NULL to omit source file information in exceptions.
            sourceURL: JSStringRef, 
            // An integer value that specifies the script's starting line number in the file at
            // sourceURL. The system only uses this when reporting exceptions.
            startingLineNumber: i32, 
            // A pointer to a JSValueRef to store an exception in, if any. 
            // Pass NULL to discard any exception.
            exception: &mut JSValueRef,
        ) -> JSValueRef
      
        /// Performs a JavaScript garbage collection.
        fn JSGarbageCollect(ctx: JSContextRef);

        //// Impls
        fn JSClassCreate();
        fn JSClassRelease();
        fn JSClassRetain();
        
        fn JSContextGetGlobalContext();
        fn JSContextGetGlobalObject();
        
        fn JSContextGetGroup();
        fn JSContextGroupCreate();
        fn JSContextGroupRelease();
        fn JSContextGroupRetain();
        
        fn JSGlobalContextCopyName();
        fn JSGlobalContextCreate();
        fn JSGlobalContextCreateInGroup();
        fn JSGlobalContextRelease();
        fn JSGlobalContextRetain();
        fn JSGlobalContextSetName();
        
        fn JSObjectCallAsConstructor();
        fn JSObjectCallAsFunction();
        
        fn JSObjectCopyPropertyNames();
        
        fn JSObjectDeleteProperty();
        fn JSObjectDeletePropertyForKey();
        
        fn JSObjectGetArrayBufferByteLength();
        fn JSObjectGetArrayBufferBytesPtr();
        fn JSObjectGetPrivate();
        fn JSObjectGetProperty();
        fn JSObjectGetPropertyAtIndex();
        fn JSObjectGetPropertyForKey();
        fn JSObjectGetPrototype();
        fn JSObjectGetTypedArrayBuffer();
        fn JSObjectGetTypedArrayByteLength();
        fn JSObjectGetTypedArrayByteOffset();
        fn JSObjectGetTypedArrayBytesPtr();
        fn JSObjectGetTypedArrayLength();
        
        fn JSObjectHasProperty();
        fn JSObjectHasPropertyForKey();
        
        fn JSObjectIsConstructor();
        fn JSObjectIsFunction();
        
        fn JSObjectMake();
        fn JSObjectMakeArray();
        fn JSObjectMakeArrayBufferWithBytesNoCopy();
        fn JSObjectMakeConstructor();
        fn JSObjectMakeDate();
        fn JSObjectMakeDeferredPromise();
        fn JSObjectMakeError();
        fn JSObjectMakeFunction();
        fn JSObjectMakeFunctionWithCallback();
        fn JSObjectMakeRegExp();
        fn JSObjectMakeTypedArray();
        fn JSObjectMakeTypedArrayWithArrayBuffer();
        fn JSObjectMakeTypedArrayWithArrayBufferAndOffset();
        fn JSObjectMakeTypedArrayWithBytesNoCopy();
        
        fn JSObjectSetPrivate();
        fn JSObjectSetProperty();
        fn JSObjectSetPropertyAtIndex();
        fn JSObjectSetPropertyForKey();
        fn JSObjectSetPrototype();
        
        fn JSStringCreateWithCharacters();
        fn JSStringCreateWithUTF8CString();
        
        fn JSStringGetCharactersPtr();
        fn JSStringGetLength();
        fn JSStringGetMaximumUTF8CStringSize();
        fn JSStringGetUTF8CString();
        
        fn JSStringIsEqual();
        fn JSStringIsEqualToUTF8CString();
        
        fn JSStringRelease();
        
        fn JSStringRetain();
        
        fn JSValueCreateJSONString();
        
        fn JSValueGetType();
        fn JSValueGetTypedArrayType();
        
        fn JSValueIsArray();
        fn JSValueIsBoolean();
        fn JSValueIsDate();
        fn JSValueIsEqual();
        fn JSValueIsInstanceOfConstructor();
        fn JSValueIsNull();
        fn JSValueIsNumber();
        fn JSValueIsObject();
        fn JSValueIsObjectOfClass();
        fn JSValueIsStrictEqual();
        fn JSValueIsString();
        fn JSValueIsSymbol();
        fn JSValueIsUndefined();
        
        fn JSValueMakeBoolean();
        fn JSValueMakeFromJSONString();
        fn JSValueMakeNull();
        fn JSValueMakeNumber();
        fn JSValueMakeString();
        fn JSValueMakeSymbol();
        fn JSValueMakeUndefined();
       
        /// Protects a JavaScript value from garbage collection.
        fn JSValueProtect(ctx: JSContextRef, val: JSValueRef);
        
        fn JSValueToBoolean();
        fn JSValueToNumber();
        fn JSValueToObject();
        fn JSValueToStringCopy();
        
        fn JSValueUnprotect();
    }
}
