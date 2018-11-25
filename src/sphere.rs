use bvh::Aabb;
use hit::{Hit, HitRecord};
use material::Material;
use math::float::consts::{FRAC_PI_2, PI};
use math::float::Float;
use ray::Ray;
use vec3::Vec3;

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
        fn calculate_hit_record<'a>(ray: &Ray, t: Float, sphere: &'a Sphere) -> HitRecord<'a> {
            let p = ray.point_at_parameter(t);
            let (u, v) = sphere_uv(&p);
            HitRecord {
                t,
                u,
                v,
                p,
                normal: (p - sphere.center_at_time(ray.time())) / sphere.radius(),
                material: &sphere.material,
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

    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let center_start = self.center_at_time(time_start);
        let aabb_0 = Aabb::new(center_start - radius, center_start + radius);
        if self.motion_vector.is_some() {
            let center_end = self.center_at_time(time_end);
            let aabb_1 = Aabb::new(center_end - radius, center_end + radius);
            Some(aabb_0.union(&aabb_1))
        } else {
            Some(aabb_0)
        }
    }
}

fn sphere_uv(p: &Vec3) -> (Float, Float) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (1. - (phi + PI) / (2. * PI), (theta + FRAC_PI_2) / PI)
}
