use crate::bvh::Aabb;
use crate::hit::{FlipNormals, Hit, HitRecord};
use crate::material::Material;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::rect::{XYRect, XZRect, YZRect};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct RectBox {
    sides: [Box<dyn Hit>; 6],
    p_min: Vec3,
    p_max: Vec3,
}

impl RectBox {
    // TODO: Instead of cloning the material we should use referencees everywhere
    pub fn new(p_min: Vec3, p_max: Vec3, material: Material) -> RectBox {
        let sides: [Box<dyn Hit>; 6] = [
            Box::new(XYRect::new(
                (p_min.x(), p_max.x()),
                (p_min.y(), p_max.y()),
                p_max.z(),
                material.clone(),
            )),
            Box::new(FlipNormals(XYRect::new(
                (p_min.x(), p_max.x()),
                (p_min.y(), p_max.y()),
                p_min.z(),
                material.clone(),
            ))),
            Box::new(XZRect::new(
                (p_min.x(), p_max.x()),
                (p_min.z(), p_max.z()),
                p_max.y(),
                material.clone(),
            )),
            Box::new(FlipNormals(XZRect::new(
                (p_min.x(), p_max.x()),
                (p_min.z(), p_max.z()),
                p_min.y(),
                material.clone(),
            ))),
            Box::new(YZRect::new(
                (p_min.y(), p_max.y()),
                (p_min.z(), p_max.z()),
                p_max.x(),
                material.clone(),
            )),
            Box::new(FlipNormals(YZRect::new(
                (p_min.y(), p_max.y()),
                (p_min.z(), p_max.z()),
                p_min.x(),
                material.clone(),
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
