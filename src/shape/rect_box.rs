use crate::bvh::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::shape::{FlipNormals, XYRect, XZRect, YZRect};
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Debug)]
pub struct RectBox {
    sides: [Box<dyn Hit>; 6],
    p_min: Vec3,
    p_max: Vec3,
}

impl RectBox {
    pub fn new(p_min: Vec3, p_max: Vec3, material: Arc<Material>) -> RectBox {
        let sides: [Box<dyn Hit>; 6] = [
            Box::new(XYRect::new(
                (p_min.x(), p_max.x()),
                (p_min.y(), p_max.y()),
                p_max.z(),
                Arc::clone(&material),
            )),
            Box::new(FlipNormals(XYRect::new(
                (p_min.x(), p_max.x()),
                (p_min.y(), p_max.y()),
                p_min.z(),
                Arc::clone(&material),
            ))),
            Box::new(XZRect::new(
                (p_min.x(), p_max.x()),
                (p_min.z(), p_max.z()),
                p_max.y(),
                Arc::clone(&material),
            )),
            Box::new(FlipNormals(XZRect::new(
                (p_min.x(), p_max.x()),
                (p_min.z(), p_max.z()),
                p_min.y(),
                Arc::clone(&material),
            ))),
            Box::new(YZRect::new(
                (p_min.y(), p_max.y()),
                (p_min.z(), p_max.z()),
                p_max.x(),
                Arc::clone(&material),
            )),
            Box::new(FlipNormals(YZRect::new(
                (p_min.y(), p_max.y()),
                (p_min.z(), p_max.z()),
                p_min.x(),
                Arc::clone(&material),
            ))),
        ];
        RectBox {
            sides,
            p_min,
            p_max,
        }
    }
}

impl Hit for RectBox {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: Float, _: Float) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}
