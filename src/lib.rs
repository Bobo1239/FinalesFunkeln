extern crate rand;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(not(feature = "f64"))]
pub mod float {
    pub use std::f32::*;
}

#[cfg(feature = "f64")]
pub type Float = f64;
#[cfg(feature = "f64")]
pub mod float {
    pub use std::f64::*;
}

pub mod camera;
pub mod hit;
pub mod image;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
