use crate::bvh::Aabb;
use crate::hit::Hit;
use crate::hit::HitRecord;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Transform: Hit + Sized {
    fn flip_normals(self) -> FlipNormals<Self>;
    fn translate(self, offset: Vec3) -> Translate<Self>;
}

impl<T: Hit> Transform for T {
    fn flip_normals(self) -> FlipNormals<T> {
        FlipNormals(self)
    }

    fn translate(self, offset: Vec3) -> Translate<T> {
        Translate {
            inner: self,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct FlipNormals<T: Hit>(pub T);

impl<T: Hit> Hit for FlipNormals<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        let mut hit_record = self.0.hit(ray, t_min, t_max);
        if let Some(hit_record) = hit_record.as_mut() {
            hit_record.normal = -hit_record.normal
        }
        hit_record
    }

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.0.bounding_box(time_start, time_end)
    }
}

#[derive(Debug)]
pub struct Translate<T: Hit> {
    inner: T,
    offset: Vec3,
}

impl<T: Hit> Hit for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        let mut hit_record = self.inner.hit(&offset_ray, t_min, t_max);
        if let Some(hit_record) = hit_record.as_mut() {
            hit_record.p += self.offset;
        }
        hit_record
    }

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.inner
            .bounding_box(time_start, time_end)
            .map(|bb| Aabb::new(bb.min + self.offset, bb.max + self.offset))
    }
}
