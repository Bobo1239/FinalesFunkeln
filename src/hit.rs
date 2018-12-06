use std::fmt::Debug;

use crate::bvh::Aabb;
use crate::material::Material;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hit: Sync + Send + Debug {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb>;
}

pub struct HitRecord<'a> {
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl Hit for [Box<Hit>] {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.iter()
            .fold((None, t_max), |(closest_hit, closest_t), item| {
                match item.hit(ray, t_min, closest_t) {
                    Some(hit_record) => {
                        let t = hit_record.t;
                        (Some(hit_record), t)
                    }
                    None => (closest_hit, closest_t),
                }
            })
            .0
    }

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut aabb = Aabb::empty();
        for hit in self {
            if let Some(new_aabb) = hit.bounding_box(time_start, time_end) {
                aabb = aabb.union(&new_aabb)
            } else {
                return None;
            }
        }

        Some(aabb)
    }
}

#[derive(Debug)]
pub struct FlipNormals<T: Hit>(pub T);

impl<T: Hit> Hit for FlipNormals<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut hit_record = self.0.hit(ray, t_min, t_max);
        hit_record
            .as_mut()
            .map(|hit_record| hit_record.normal = -hit_record.normal);
        hit_record
    }

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.0.bounding_box(time_start, time_end)
    }
}
