#[cfg(not(feature = "f64"))]
pub mod float {
    pub type Float = f32;
    pub use std::f32::*;
}

#[cfg(feature = "f64")]
pub mod float {
    pub type Float = f64;
    pub use std::f64::*;
}

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
