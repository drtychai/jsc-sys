#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

//pub use generated::*;
//mod generated { include!(concat!(env!("OUT_DIR"), "/build/bindings.rs")); }
include!(concat!(env!("OUT_DIR"), "/build/bindings.rs"));
