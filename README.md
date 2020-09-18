# jsc-sys
A `*-sys` crate of raw bindings to the [JavaScriptCore][1] low-level C API.

[![crates.io-badge]][crates.io] [![docs.rs-badge]][docs.rs] [![license]][lic]

[![gnu-badge]][gnu-build] [![musl-badge]][musl-build] [![darwin-badge]][darwin-build]


[crates.io]: https://crates.io/crates/jscjs-sys
[crates.io-badge]:  https://img.shields.io/crates/v/jscjs-sys.svg

[docs.rs]: https://docs.rs/jscjs-sys
[docs.rs-badge]: https://docs.rs/jscjs-sys/badge.svg

[gnu-build]: https://github.com/drtychai/jsc-sys/actions?query=workflow:gnu
[gnu-badge]: https://github.com/drtychai/jsc-sys/workflows/gnu/badge.svg

[musl-build]: https://github.com/drtychai/jsc-sys/actions?query=workflow:musl
[musl-badge]: https://github.com/drtychai/jsc-sys/workflows/musl/badge.svg

[darwin-build]: https://github.com/drtychai/jsc-sys/actions?query=workflow:darwin
[darwin-badge]: https://github.com/drtychai/jsc-sys/workflows/darwin/badge.svg

[license]: https://img.shields.io/crates/l/jscjs-sys.svg
[lic]: /LICENSE

## Building
Prerequisites libraries:
- LLVM toolchain (`llvm-dev` or `clang-dev`)
- CMake and Make utilities
- WebKit dependencies (GNU/Linux specific):
  - `libicu-dev`
  - `libgcrypt20-dev`


```sh
#
#  Clone crate and WebKit source (included as a submodule)
#
➜ git clone --recursive https://github.com/drtychai/jsc-sys

                 [ ... ]

#
#  No special caveats to builds
#
➜ cargo build [-vv] [--target <TRIPLE>]

                 [ ... ]

#
#  Packaging requires the `JSC_SRC` environment variable
#
➜ export JSC_SRC=/abs/path/to/webkit && cargo package [-vv] [--target <TRIPLE>]
```


## Usage
Add the following to your `Cargo.toml`:

```toml
[dependencies]
jscjs-sys = "0.0.2"
```

All necessary definitions are provided to easily interoperate with the JSC API on all `x86_64` *nix
architectures supported by Rust. This crate explicity exports:

|   libJavaScriptCore     |      jscjs_sys         |
|:-----------------------:|:----------------------:|
|  `JSContextGroupRef`    |  `jscjs_sys::VM`       |
|  `JSGlobalContextRef`   |  `jscjs_sys::Context`  |
|  `JSString`             |  `jscjs_sys::Sting`    |
|  `JSValueRef`           |  `jscjs_sys::Value`    |
|  `JSObjectref`          |  `jscjs_sys::Object`   |


These bindings are designed to be a fairly straightforward translation to the low-level C API,
while taking advantage of Rust's memory safety. For more about the JavaScriptCore API, see the
API [source][2] and the [documentation][3].

Provided below are some practical examples of what functionalities this crate allows:
  - Create a global scripting context, used to create and execute JavaScript objects and code
  - Work natively with objects, parameters
  - Build JavaScript functions out of strings
  - Associate C callbacks to user-definted "classes" of objects
  - Attach C callbacks to "classes", handles responses to an action (e.g., getters/setters, promises, fn cals)
  - Load JavaScript files based on designated names and starting line-numbers

[1]: https://trac.webkit.org/wiki/JavaScriptCore
[2]: https://github.com/WebKit/webkit/tree/master/Source/JavaScriptCore/API
[3]: https://developer.apple.com/documentation/javascriptcore
