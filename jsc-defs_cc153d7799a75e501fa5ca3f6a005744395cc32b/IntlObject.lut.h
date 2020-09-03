// Automatically generated from JavaScriptCore/runtime/IntlObject.cpp using JavaScriptCore/create_hash_table. DO NOT EDIT!

#include "Lookup.h"

namespace JSC {

static const struct CompactHashIndex intlObjectTableIndex[17] = {
    { 1, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 4, -1 },
    { 2, -1 },
    { 7, -1 },
    { 3, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 5, -1 },
    { -1, -1 },
    { -1, -1 },
    { -1, -1 },
    { 0, 16 },
    { 6, -1 },
};

static const struct HashTableValue intlObjectTableValues[8] = {
   { "getCanonicalLocales", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::Function), NoIntrinsic, { (intptr_t)static_cast<RawNativeFunction>(intlObjectFuncGetCanonicalLocales), (intptr_t)(1) } },
   { "Collator", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createCollatorConstructor), (intptr_t)(0) } },
   { "DateTimeFormat", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createDateTimeFormatConstructor), (intptr_t)(0) } },
   { "Locale", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createLocaleConstructor), (intptr_t)(0) } },
   { "NumberFormat", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createNumberFormatConstructor), (intptr_t)(0) } },
   { "PluralRules", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createPluralRulesConstructor), (intptr_t)(0) } },
   { "RelativeTimeFormat", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createRelativeTimeFormatConstructor), (intptr_t)(0) } },
   { "Segmenter", static_cast<unsigned>(PropertyAttribute::DontEnum|PropertyAttribute::PropertyCallback), NoIntrinsic, { (intptr_t)static_cast<LazyPropertyCallback>(createSegmenterConstructor), (intptr_t)(0) } },
};

static const struct HashTable intlObjectTable =
    { 8, 15, false, nullptr, intlObjectTableValues, intlObjectTableIndex };

} // namespace JSC
