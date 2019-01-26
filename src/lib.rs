use std::fmt::Debug;

pub mod bvh;
pub mod camera;
pub mod hit;
pub mod image;
pub mod material;
pub mod math;
pub mod perlin;
pub mod ray;
pub mod shape;
pub mod texture;
pub mod vec3;

pub trait Rng: 'static + rand::Rng + Debug + Send + Sync {}

impl<T: 'static + rand::Rng + Debug + Send + Sync> Rng for T {}
