// Automatically generated from JavaScriptCore/runtime/JSDataViewPrototype.cpp using JavaScriptCore/create_hash_table. DO NOT EDIT!

#include "Lookup.h"

namespace JSC {

static const struct CompactHashIndex dataViewTableIndex[68] = {
    { -1, -1 },
    { -1, -1 },
    { 18, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 10, -1 },
    { -1, -1 },
    { 16, -1 },
    { -1, -1 },
    { 17, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 9, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 0, 66 },
    { -1, -1 },
    { 1, 67 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 6, -1 },
    { -1, -1 },
    { -1, -1 },
    { 13, -1 },
    { 2, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 5, 64 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 12, -1 },
    { 3, 65 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 4, -1 },
    { -1, -1 },
    { 11, -1 },
    { -1, -1 },
    { 7, -1 },
    { 8, -1 },
    { 14, -1 },
    { 15, -1 },
};

static const struct HashTableValue dataViewTableValues[19] = {
   { "getInt8", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetInt8, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetInt8), (intptr_t)(1) } },
   { "getUint8", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetUint8, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetUint8), (intptr_t)(1) } },
   { "getInt16", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetInt16, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetInt16), (intptr_t)(1) } },
   { "getUint16", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetUint16, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetUint16), (intptr_t)(1) } },
   { "getInt32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetInt32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetInt32), (intptr_t)(1) } },
   { "getUint32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetUint32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetUint32), (intptr_t)(1) } },
   { "getFloat32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetFloat32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetFloat32), (intptr_t)(1) } },
   { "getFloat64", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewGetFloat64, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncGetFloat64), (intptr_t)(1) } },
   { "setInt8", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetInt8, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetInt8), (intptr_t)(2) } },
   { "setUint8", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetUint8, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetUint8), (intptr_t)(2) } },
   { "setInt16", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetInt16, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetInt16), (intptr_t)(2) } },
   { "setUint16", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetUint16, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetUint16), (intptr_t)(2) } },
   { "setInt32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetInt32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetInt32), (intptr_t)(2) } },
   { "setUint32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetUint32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetUint32), (intptr_t)(2) } },
   { "setFloat32", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetFloat32, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetFloat32), (intptr_t)(2) } },
   { "setFloat64", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), DataViewSetFloat64, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoFuncSetFloat64), (intptr_t)(2) } },
   { "buffer", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Accessor), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoGetterBuffer), (intptr_t)static_cast<RawNativeFunction>(nullptr) } },
   { "byteLength", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Accessor), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoGetterByteLength), (intptr_t)static_cast<RawNativeFunction>(nullptr) } },
   { "byteOffset", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Accessor), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(dataViewProtoGetterByteOffset), (intptr_t)static_cast<RawNativeFunction>(nullptr) } },
};

static const struct HashTable dataViewTable =
    { 19, 63, true, nullptr, dataViewTableValues, dataViewTableIndex };

} // namespace JSC
