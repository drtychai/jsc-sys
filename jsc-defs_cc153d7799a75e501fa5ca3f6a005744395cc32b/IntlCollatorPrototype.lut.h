// Automatically generated from JavaScriptCore/runtime/IntlCollatorPrototype.cpp using JavaScriptCore/create_hash_table. DO NOT EDIT!

#include "Lookup.h"

namespace JSC {

static const struct CompactHashIndex collatorPrototypeTableIndex[4] = {
    { 0, -1 },
    { -1, -1 },
    { -1, -1 },
    { 1, -1 },
};

static const struct HashTableValue collatorPrototypeTableValues[2] = {
   { "compare", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Accessor), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(IntlCollatorPrototypeGetterCompare), (intptr_t)static_cast<RawNativeFunction>(nullptr) } },
   { "resolvedOptions", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(IntlCollatorPrototypeFuncResolvedOptions), (intptr_t)(0) } },
};

static const struct HashTable collatorPrototypeTable =
    { 2, 3, true, nullptr, collatorPrototypeTableValues, collatorPrototypeTableIndex };

} // namespace JSC
