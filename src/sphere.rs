use hit::{Hit, HitRecord};
use material::Material;
use ray::Ray;
use vec3::Vec3;
use Float;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Material,
    motion_vector: Option<Vec3>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            motion_vector: None,
        }
    }

    pub fn new_moving(
        center: Vec3,
        radius: Float,
        material: Material,
        motion_vector: Vec3,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            motion_vector: Some(motion_vector),
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn center_at_time(&self, t: Float) -> Vec3 {
        if let Some(motion_vector) = self.motion_vector {
            self.center + t * motion_vector
        } else {
            self.center
        }
    }

    pub fn radius(&self) -> Float {
        self.radius
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        fn calculate_hit_record<'a>(ray: &Ray, t: Float, s: &'a Sphere) -> HitRecord<'a> {
            let p = ray.point_at_parameter(t);
            HitRecord {
                t,
                p,
                normal: (p - s.center_at_time(ray.time())) / s.radius(),
                material: &s.material,
            }
        }

        let oc = ray.origin() - self.center_at_time(ray.time());
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                return Some(calculate_hit_record(ray, t, self));
            }
            t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                return Some(calculate_hit_record(ray, t, self));
            }
        }
        None
    }
}
