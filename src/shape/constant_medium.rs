use std::marker::PhantomData;

use crate::bvh::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::math::float::{self, Float};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::Rng;

#[derive(Debug)]
pub struct ConstantMedium<R: Rng, T: Hit<R>> {
    boundary: T,
    density: Float,
    phase_function: Material,
    phantom_data: PhantomData<R>,
}

impl<R: Rng, T: Hit<R>> ConstantMedium<R, T> {
    pub fn new(boundary: T, density: Float, albedo: Texture) -> ConstantMedium<R, T> {
        ConstantMedium {
            boundary,
            density,
            phase_function: Material::isotropic(albedo),
            phantom_data: PhantomData,
        }
    }
}

impl<R: Rng, T: Hit<R>> Hit<R> for ConstantMedium<R, T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut R) -> Option<HitRecord<'_>> {
        if let Some(mut rec1) = self.boundary.hit(ray, float::MIN, float::MAX, rng) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, float::MAX, rng) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t < rec2.t {
                    if rec1.t < 0. {
                        rec1.t = 0.;
                    }
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray.direction().length();
                    let hit_distance = -(1. / self.density) * rng.gen::<Float>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / ray.direction().length();
                        return Some(HitRecord {
                            t,
                            p: ray.point_at_parameter(t),
                            normal: Vec3::new(1., 0., 0.), // arbitrary
                            material: &(self.phase_function),
                            u: 0.0,
                            v: 0.0,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.boundary.bounding_box(time_start, time_end)
    }
}
