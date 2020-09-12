/*
 * GLib public API headers
 */
#ifndef __JSC_H__
#define __JSC_H__

#define __JSC_H_INSIDE__

#include <jsc/JSCClass.h>
#include <jsc/JSCContext.h>
#include <jsc/JSCDefines.h>
#include <jsc/JSCException.h>
#include <jsc/JSCOptions.h>
#include <jsc/JSCValue.h>
#include <jsc/JSCVersion.h>
#include <jsc/JSCVirtualMachine.h>
#include <jsc/JSCWeakValue.h>

#include <jsc/JSCAutocleanups.h>

#undef __JSC_H_INSIDE__
#endif /* __JSC_H__ */

/*
 * libJavaScriptCore headers
 * --> imported headers are only utilized on apple-darwin builds
 */
#ifndef JavaScriptCore_h
#define JavaScriptCore_h

#include <JavaScriptCore/JavaScript.h>
#include <JavaScriptCore/JSStringRefCF.h>

#if defined(__OBJC__) && JSC_OBJC_API_ENABLED
/* Only generate bindings for these if Obj-C API is enabled (e.g., for macos) */

#import "JSContext.h"
#import "JSValue.h"
#import "JSManagedValue.h"
#import "JSVirtualMachine.h"
#import "JSExport.h"

#endif

#endif /* JavaScriptCore_h */
