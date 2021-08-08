mod prefix {
    include!(concat!(env!("OUT_DIR"), "/build/bindings.rs"));
}

pub use self::prefix::*;
