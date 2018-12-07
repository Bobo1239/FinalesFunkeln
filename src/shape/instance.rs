use crate::bvh::Aabb;
use crate::hit::Hit;
use crate::hit::HitRecord;
use crate::math::float::{self, Float};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Transform: Hit + Sized {
    fn flip_normals(self) -> FlipNormals<Self>;
    fn translate(self, offset: Vec3) -> Translate<Self>;
    fn rotate_y(self, angle: Float) -> RotateY<Self>;
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

    fn rotate_y(self, angle: Float) -> RotateY<T> {
        RotateY::new(self, angle)
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

#[derive(Debug)]
pub struct RotateY<T: Hit> {
    inner: T,
    bounding_box: Option<Aabb>,
    sin_theta: Float,
    cos_theta: Float,
}

impl<T: Hit> RotateY<T> {
    pub fn new(inner: T, angle: Float) -> RotateY<T> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        RotateY {
            sin_theta,
            cos_theta,
            bounding_box: inner.bounding_box(0., 1.).map(|bb| {
                let mut min = Vec3::new(float::MAX, float::MAX, float::MAX);
                let mut max = Vec3::new(-float::MAX, -float::MAX, -float::MAX);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bb.max.x() + (1 - i) as f32 * bb.min.x();
                            let y = j as f32 * bb.max.y() + (1 - j) as f32 * bb.min.y();
                            let z = k as f32 * bb.max.z() + (1 - k) as f32 * bb.min.z();
                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(new_x, y, new_z);
                            for c in 0..2 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c]
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c]
                                }
                            }
                        }
                    }
                }
                Aabb::new(min, max)
            }),
            inner,
        }
    }
}

impl<T: Hit> Hit for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated = Ray::new(origin, direction, ray.time());
        self.inner
            .hit(&rotated, t_min, t_max)
            .map(|mut hit_record| {
                let mut p = hit_record.p;
                let mut normal = hit_record.normal;

                p[0] = self.cos_theta * hit_record.p[0] + self.sin_theta * hit_record.p[2];
                p[2] = -self.sin_theta * hit_record.p[0] + self.cos_theta * hit_record.p[2];

                normal[0] =
                    self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
                normal[2] =
                    -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

                hit_record.p = p;
                hit_record.normal = normal;
                hit_record
            })
    }

    fn bounding_box(&self, _: Float, _: Float) -> Option<Aabb> {
        self.bounding_box
    }
}
