/// Rust FFI bindings to JavaScriptCore's CXX runtime API
#[cxx::bridge]
mod ffi {
    // Shared structs with fields visible to both languages.
    //  -----

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        //include!("WebKit/Source/JavaScriptCore/API/JavaScript.h");

        //// Structs
        /// A structure that contains properties and callbacks that define a type of object.
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
        /// Constants that identify the type of a JavaScript value.
        type JSType;
        /// The type of a JavaScript typed array object.
        type JSTypedArrayType;

        //// Typedefs
        /// A Unicode character.
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

        /// A JavaScript property attribute.
        type JSPropertyAttributes;
        
        /// An ordered set of the names of a JavaScript object’s properties.
        type JSPropertyNameAccumulatorRef;
        
        /// An array of JavaScript property names.
        type JSPropertyNameArrayRef;
        
        /// A UTF-16 character buffer.
        type JSStringRef;
        
        /// A defntion that deallocates bytes that pass to a typed array constructor.;
        type JSTypedArrayBytesDeallocator;
        
        /// A JavaScript value.
        type JSValueRef;

        ///// Script Functions
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
      
        //// Performs a JavaScript garbage collection.
        fn JSGarbageCollect(ctx: JSContextRef);

        ///// Impls        
        /// Returns a JavaScript value’s type.
        fn JSValueGetType(JSContextRef, JSValueRef) -> JSType;
        
        /// Tests whether a JavaScript value’s type is the undefined type.
        fn JSValueIsUndefined(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value’s type is the null type.
        fn JSValueIsNull(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value is Boolean.
        fn JSValueIsBoolean(JSContextRef, JSValueRef) -> Bool;
        

        /// Tests whether a JavaScript value’s type is the number type.
        fn JSValueIsNumber(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value’s type is the string type.
        fn JSValueIsString(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value's type is the symbol type.
        fn JSValueIsSymbol(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value’s type is the object type.
        fn JSValueIsObject(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value is an object with a specified class in its class chain.
        fn JSValueIsObjectOfClass(JSContextRef, JSValueRef, JSClassRef) -> Bool;
        
        /// Tests whether a JavaScript value is an array.
        fn JSValueIsArray(JSContextRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value is a date.
        fn JSValueIsDate(JSContextRef, JSValueRef) -> Bool;
        
        /// Returns a JavaScript value’s typed array type.
        fn JSValueGetTypedArrayType(JSContextRef, JSValueRef, &mut JSValueRef) -> JSTypedArrayType;
       
        /// Creates a JavaScript value of the undefined type.
        fn JSValueMakeUndefined(JSContextRef) -> JSValueRef;
        
        /// Creates a JavaScript value of the null type.
        fn JSValueMakeNull(JSContextRef) -> JSValueRef;
        
        /// Creates a JavaScript Boolean value.
        fn JSValueMakeBoolean(JSContextRef, Bool) -> JSValueRef;
        
        /// Creates a JavaScript value of the number type.
        fn JSValueMakeNumber(JSContextRef, Double) -> JSValueRef;
        
        /// Creates a JavaScript value of the string type.
        fn JSValueMakeString(JSContextRef, JSStringRef) -> JSValueRef;
        
        /// Creates a JavaScript value of the symbol type.
        fn JSValueMakeSymbol(JSContextRef, JSStringRef) -> JSValueRef;
        
        /// Converts a JavaScript value to a Boolean and returns the resulting Boolean.
        fn JSValueToBoolean(JSContextRef, JSValueRef) -> Bool;
        
        /// Converts a JavaScript value to a number and returns the resulting number.
        fn JSValueToNumber(JSContextRef, JSValueRef, &mut JSValueRef) -> Double;
        
        /// Converts a JavaScript value to a string and copies the result into a JavaScript string.
        fn JSValueToStringCopy(JSContextRef, JSValueRef, &mut JSValueRef) -> JSStringRef;
        
        /// Converts a JavaScript value to an object and returns the resulting object.
        fn JSValueToObject(JSContextRef, JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript value from a JSON-formatted string.
        fn JSValueMakeFromJSONString(JSContextRef, JSStringRef) -> JSValueRef;
        
        /// Creates a JavaScript string that contains the JSON-serialized representation of a JavaScript value.
        fn JSValueCreateJSONString(JSContextRef, JSValueRef, UInt32, &mut JSValueRef) -> JSStringRef;
        
        /// Tests whether two JavaScript values are equal.
        fn JSValueIsEqual(JSContextRef, JSValueRef, JSValueRef, &mut JSValueRef) -> Bool;
        
        /// Tests whether two JavaScript values are strict equal.
        fn JSValueIsStrictEqual(JSContextRef, JSValueRef, JSValueRef) -> Bool;
        
        /// Tests whether a JavaScript value is an object that the specified constructor creates.
        fn JSValueIsInstanceOfConstructor(JSContextRef, JSValueRef, JSObjectRef, &mut JSValueRef) -> Bool;
        
        /// Protects a JavaScript value from garbage collection.
        fn JSValueProtect(JSContextRef, JSValueRef);
        
        /// Unprotects a JavaScript value from garbage collection.
        fn JSValueUnprotect(JSContextRef, JSValueRef);
        
        /// Gets the global object of a JavaScript execution context.
        fn JSContextGetGlobalObject(JSContextRef) -> JSObjectRef;
        
        /// Calls an object as a constructor.
        fn JSObjectCallAsConstructor(JSContextRef, JSObjectRef, Int, &JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Calls an object as a fntion.;
        fn JSObjectCallAsFunction(JSContextRef, JSObjectRef, JSObjectRef, Int, &JSValueRef, &mut JSValueRef) -> JSValueRef;
        
        /// Gets the names of an object’s enumerable properties.
        fn JSObjectCopyPropertyNames(JSContextRef, JSObjectRef) -> JSPropertyNameArrayRef;
        
        /// Deletes a property from an object.
        fn JSObjectDeleteProperty(JSContextRef, JSObjectRef, JSStringRef, &mut JSValueRef) -> Bool;
        
        /// Gets an object’s private data.
        fn JSObjectGetPrivate(JSObjectRef) -> UnsafeMutableRawPointer;
        
        /// Gets a property from an object.
        fn JSObjectGetProperty(JSContextRef, JSObjectRef, JSStringRef, &mut JSValueRef) -> JSValueRef;
        
        /// Gets a property from an object by numeric index.
        fn JSObjectGetPropertyAtIndex(JSContextRef, JSObjectRef, UInt32, &mut JSValueRef) -> JSValueRef;
        
        /// Gets an object’s prototype.
        fn JSObjectGetPrototype(JSContextRef, JSObjectRef) -> JSValueRef;
        
        /// Tests whether an object has a specified property.
        fn JSObjectHasProperty(JSContextRef, JSObjectRef, JSStringRef) -> Bool;
        
        /// Tests whether you can call an object as a constructor.
        fn JSObjectIsConstructor(JSContextRef, JSObjectRef) -> Bool;
        
        /// Tests whether you can call an object as a fntion.;
        fn JSObjectIsFunction(JSContextRef, JSObjectRef) -> Bool;
        
        /// Creates a JavaScript object.
        fn JSObjectMake(JSContextRef, JSClassRef, Box<_>) -> JSObjectRef;
        
        /// Creates a JavaScript array object.
        fn JSObjectMakeArray(JSContextRef, Int, &JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript constructor.
        fn JSObjectMakeConstructor(JSContextRef, JSClassRef, JSObjectCallAsConstructorCallback) -> JSObjectRef;
        
        /// Creates a JavaScript date object as though invoking the built-in date constructor.
        fn JSObjectMakeDate(JSContextRef, Int, &JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript error object as though invoking the built-in error constructor.
        fn JSObjectMakeError(JSContextRef, Int, &JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a fntion with a specified script as its body.;
        fn JSObjectMakeFunction(JSContextRef, JSStringRef, UInt32, &JSStringRef, JSStringRef, JSStringRef, Int32, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript fntion with a specified callback as its implementation.;
        fn JSObjectMakeFunctionWithCallback(JSContextRef, JSStringRef, JSObjectCallAsFunctionCallback) -> JSObjectRef;
        
        /// Creates a JavaScript regular expression object as though invoking the built-in regular expression constructor.
        fn JSObjectMakeRegExp(JSContextRef, Int, &JSValueRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Sets a pointer to private data on an object.
        fn JSObjectSetPrivate(JSObjectRef, Box<_>) -> Bool;
        
        /// Sets a property on an object.
        fn JSObjectSetProperty(JSContextRef, JSObjectRef, JSStringRef, JSValueRef, JSPropertyAttributes, &mut JSValueRef);
        
        /// Sets a property on an object by numeric index.
        fn JSObjectSetPropertyAtIndex(JSContextRef, JSObjectRef, UInt32, JSValueRef, &mut JSValueRef);
        
        /// Gets a property from an object using a JavaScript value as the property key.
        fn JSObjectGetPropertyForKey(JSContextRef, JSObjectRef, JSValueRef, &mut JSValueRef) -> JSValueRef;
        
        /// Sets an object’s prototype.
        fn JSObjectSetPrototype(JSContextRef, JSObjectRef, JSValueRef);
        
        /// Deletes a property from an object using a JavaScript value as the property key.
        fn JSObjectDeletePropertyForKey(JSContextRef, JSObjectRef, JSValueRef, &mut JSValueRef) -> Bool;
        
        /// Tests whether an object has the specified property using a JavaScript value as the property key.
        fn JSObjectHasPropertyForKey(JSContextRef, JSObjectRef, JSValueRef, &mut JSValueRef) -> Bool;
        
        /// Sets a property on an object using a JavaScript value as the property key.
        fn JSObjectSetPropertyForKey(JSContextRef, JSObjectRef, JSValueRef, JSValueRef, JSPropertyAttributes, &mut JSValueRef);
        
        /// Creates a JavaScript promise object by invoking the provided executor.
        fn JSObjectMakeDeferredPromise(JSContextRef, &mut JSObjectRef, &mut JSObjectRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript class.
        fn JSClassCreate(&JSClassDefinition) -> JSClassRef;
        
        /// Releases a JavaScript class.
        fn JSClassRelease(JSClassRef);
        
        /// Retains a JavaScript class.
        fn JSClassRetain(JSClassRef) -> JSClassRef;
       
        /// Adds a property name to a JavaScript property name accumulator.
        fn JSPropertyNameAccumulatorAddName(JSPropertyNameAccumulatorRef, JSStringRef);
        
        /// Gets a count of the number of items in a JavaScript property name array.
        fn JSPropertyNameArrayGetCount(JSPropertyNameArrayRef) -> Int;
        
        /// Gets a property name at a specified index in a JavaScript property name array.
        fn JSPropertyNameArrayGetNameAtIndex(JSPropertyNameArrayRef, Int) -> JSStringRef;
        
        /// Releases a JavaScript property name array.
        fn JSPropertyNameArrayRelease(JSPropertyNameArrayRef);
        
        /// Retains a JavaScript property name array.
        fn JSPropertyNameArrayRetain(JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;
        
        /// Creates a JavaScript typed array object with the specified number of elements.
        fn JSObjectMakeTypedArray(JSContextRef, JSTypedArrayType, Int, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript typed array object from an existing pointer.
        fn JSObjectMakeTypedArrayWithBytesNoCopy(JSContextRef, JSTypedArrayType, UnsafeMutableRawPointer, Int, JSTypedArrayBytesDeallocator, UnsafeMutableRawPointer, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript typed array object from an existing JavaScript array buffer object.
        fn JSObjectMakeTypedArrayWithArrayBuffer(JSContextRef, JSTypedArrayType, JSObjectRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript typed array object from an existing JavaScript array buffer object with the specified offset and length.
        fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(JSContextRef, JSTypedArrayType, JSObjectRef, Int, Int, &mut JSValueRef) -> JSObjectRef;
        /// Returns a temporary pointer to the backing store of a JavaScript typed array object.
        fn JSObjectGetTypedArrayBytesPtr(JSContextRef, JSObjectRef, &mut JSValueRef) -> UnsafeMutableRawPointer;
        
        /// Returns the length of a JavaScript typed array object.
        fn JSObjectGetTypedArrayLength(JSContextRef, JSObjectRef, &mut JSValueRef) -> Int;
        
        /// Returns the byte length of a JavaScript typed array object.
        fn JSObjectGetTypedArrayByteLength(JSContextRef, JSObjectRef, &mut JSValueRef) -> Int;
        
        /// Returns the byte offset of a JavaScript typed array object.
        fn JSObjectGetTypedArrayByteOffset(JSContextRef, JSObjectRef, &mut JSValueRef) -> Int;
        
        /// Returns the JavaScript array buffer object to use as the backing of a JavaScript typed array object.
        fn JSObjectGetTypedArrayBuffer(JSContextRef, JSObjectRef, &mut JSValueRef) -> JSObjectRef;
        
        /// Creates a JavaScript array buffer object from an existing pointer.
        fn JSObjectMakeArrayBufferWithBytesNoCopy(
            JSContextRef, 
            UnsafeMutableRawPointer, 
            i32,
            JSTypedArrayBytesDeallocator, 
            UnsafeMutableRawPointer,
            &mut JSValueRef
        ) -> JSObjectRef;
        
        /// Returns the number of bytes in a JavaScript data object.
        fn JSObjectGetArrayBufferByteLength(JSContextRef, JSObjectRef, &mut JSValueRef) -> Int;
        
        /// Returns a pointer to the data buffer that serves as the backing store for a JavaScript typed array object.
        fn JSObjectGetArrayBufferBytesPtr(JSContextRef, JSObjectRef, &mut JSValueRef) -> UnsafeMutableRawPointer;
        
        /// Creates a JavaScript string from a buffer of Unicode characters.
        fn JSStringCreateWithCharacters(&JSChar, i32) -> JSStringRef;
       
        /// Creates a JavaScript string from a null-terminated UTF-8 string.
        fn JSStringCreateWithUTF8CString(&[u8]) -> JSStringRef;
        
        /// Retains a JavaScript string.
        fn JSStringRetain(JSStringRef) -> JSStringRef;
        
        /// Releases a JavaScript string.
        fn JSStringRelease(JSStringRef);
        
        /// Returns the number of Unicode characters in a JavaScript string.
        fn JSStringGetLength(JSStringRef) -> u32;
        
        /// Returns a pointer to the Unicode character buffer that serves as the backing store for a JavaScript string.
        fn JSStringGetCharactersPtr(JSStringRef) -> &JSChar;
        
        /// Returns the maximum number of bytes a JavaScript string uses when you convert it into a null-terminated UTF-8 string.
        fn JSStringGetMaximumUTF8CStringSize(JSStringRef) -> i32;
        
        /// Converts a JavaScript string into a null-terminated UTF-8 string, and copies the result into an external byte buffer.
        fn JSStringGetUTF8CString(JSStringRef, &[u8], i32) -> i32;
        
        /// Tests whether two JavaScript strings match.
        fn JSStringIsEqual(JSStringRef, JSStringRef) -> bool;
        
        /// Tests whether a JavaScript string matches a null-terminated UTF-8 string.
        fn JSStringIsEqualToUTF8CString(JSStringRef, &[u8]) -> bool;
        
        /// Creates a global JavaScript execution context.
        fn JSGlobalContextCreate(JSClassRef) -> JSGlobalContextRef;
        
        /// Creates a global JavaScript execution context in the provided context group.
        fn JSGlobalContextCreateInGroup(JSContextGroupRef, JSClassRef) -> JSGlobalContextRef;
        
        /// Retains a global JavaScript execution context.
        fn JSGlobalContextRetain(JSGlobalContextRef) -> JSGlobalContextRef;
        
        /// Releases a global JavaScript execution context.
        fn JSGlobalContextRelease(JSGlobalContextRef);
        
        /// Gets a copy of the name of a context.
        fn JSGlobalContextCopyName(JSGlobalContextRef) -> JSStringRef;
        
        /// Sets the remote debugging name for a context.
        fn JSGlobalContextSetName(JSGlobalContextRef, JSStringRef);
        
        /// Creates a JavaScript context group.
        fn JSContextGroupCreate() -> JSContextGroupRef;
        
        /// Retains a JavaScript context group.
        fn JSContextGroupRetain(JSContextGroupRef) -> JSContextGroupRef;
        
        /// Releases a JavaScript context group.
        fn JSContextGroupRelease(JSContextGroupRef);
        
        /// Gets the global context of a JavaScript execution context.
        fn JSContextGetGlobalContext(JSContextRef) -> JSGlobalContextRef;
        
        /// Gets the context group that a JavaScript execution context belongs to.
        fn JSContextGetGroup(JSContextRef) -> JSContextGroupRef;
    }
}
