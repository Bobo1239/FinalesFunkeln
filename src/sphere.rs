use hit::{Hit, HitRecord};
use material::Material;
use ray::Ray;
use vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        fn calculate_hit_record<'a>(ray: &Ray, t: f32, s: &'a Sphere) -> HitRecord<'a> {
            let p = ray.point_at_parameter(t);
            HitRecord {
                t,
                p,
                normal: (p - s.center()) / s.radius(),
                material: &s.material,
            }
        }

        let oc = ray.origin() - self.center;
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
