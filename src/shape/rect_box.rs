use std::fmt::Debug;
use std::sync::Arc;

use crate::bvh::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::shape::Transform;
use crate::shape::{XYRect, XZRect, YZRect};
use crate::vec3::Vec3;
use crate::Rng;

#[derive(Debug)]
pub struct RectBox<R: Rng> {
    sides: [Box<dyn Hit<R>>; 6],
    p_min: Vec3,
    p_max: Vec3,
}

impl<R: Rng> RectBox<R> {
    pub fn new(p_min: Vec3, p_max: Vec3, material: Arc<Material>) -> RectBox<R> {
        let sides: [Box<dyn Hit<R>>; 6] = [
            Box::new(XYRect::new(
                (p_min.x(), p_max.x()),
                (p_min.y(), p_max.y()),
                p_max.z(),
                Arc::clone(&material),
            )),
            Box::new(
                XYRect::new(
                    (p_min.x(), p_max.x()),
                    (p_min.y(), p_max.y()),
                    p_min.z(),
                    Arc::clone(&material),
                )
                .flip_normals(),
            ),
            Box::new(XZRect::new(
                (p_min.x(), p_max.x()),
                (p_min.z(), p_max.z()),
                p_max.y(),
                Arc::clone(&material),
            )),
            Box::new(
                XZRect::new(
                    (p_min.x(), p_max.x()),
                    (p_min.z(), p_max.z()),
                    p_min.y(),
                    Arc::clone(&material),
                )
                .flip_normals(),
            ),
            Box::new(YZRect::new(
                (p_min.y(), p_max.y()),
                (p_min.z(), p_max.z()),
                p_max.x(),
                Arc::clone(&material),
            )),
            Box::new(
                YZRect::new(
                    (p_min.y(), p_max.y()),
                    (p_min.z(), p_max.z()),
                    p_min.x(),
                    Arc::clone(&material),
                )
                .flip_normals(),
            ),
        ];
        RectBox {
            sides,
            p_min,
            p_max,
        }
    }
}

impl<R: Rng + Debug> Hit<R> for RectBox<R> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rng: &mut R) -> Option<HitRecord<'_>> {
        self.sides.hit(ray, t_min, t_max, rng)
    }

    fn bounding_box(&self, _: Float, _: Float) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}
