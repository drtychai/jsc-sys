// Automatically generated from JavaScriptCore/wasm/js/WebAssemblyModuleConstructor.cpp using JavaScriptCore/create_hash_table. DO NOT EDIT!

#include "Lookup.h"

namespace JSC {

static const struct CompactHashIndex constructorTableWebAssemblyModuleIndex[8] = {
    { 2, -1 },
    { 0, -1 },
    { 1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
};

static const struct HashTableValue constructorTableWebAssemblyModuleValues[3] = {
   { "customSections", static_cast<unsigned>(PropertyAttribute::Function), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(webAssemblyModuleCustomSections), (intptr_t)(2) } },
   { "imports", static_cast<unsigned>(PropertyAttribute::Function), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(webAssemblyModuleImports), (intptr_t)(1) } },
   { "exports", static_cast<unsigned>(PropertyAttribute::Function), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(webAssemblyModuleExports), (intptr_t)(1) } },
};

static const struct HashTable constructorTableWebAssemblyModule =
    { 3, 7, false, nullptr, constructorTableWebAssemblyModuleValues, constructorTableWebAssemblyModuleIndex };

} // namespace JSC
