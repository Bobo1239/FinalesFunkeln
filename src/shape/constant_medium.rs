use crate::hit::Hit;
use crate::material::Isotropic;
use crate::math::float::Float;
use crate::Rng;
use std::marker::PhantomData;

pub struct ConstantMedium<R: Rng, T: Hit<R>> {
    inner: T,
    phase_function: Isotropic,
    density: Float,
    phantom_data: PhantomData<R>,
}
