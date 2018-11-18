use hit::HitRecord;
use ray::Ray;
use vec3::Vec3;

use rand::Rng;

trait Scatter {
    fn scatter(&self, ray: &Ray, attenuation: &Vec3, hit_record: &HitRecord)
        -> Option<(Ray, Vec3)>;
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        _attenuation: &Vec3,
        hit_record: &HitRecord,
    ) -> Option<(Ray, Vec3)> {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_sphere();
        let scattered: Ray = Ray::new(hit_record.p, target - hit_record.p);
        Some((scattered, self.albedo))
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        _attenuation: &Vec3,
        hit_record: &HitRecord,
    ) -> Option<(Ray, Vec3)> {
        let refleced: Vec3 = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        let scattered: Ray = Ray::new(hit_record.p, refleced);
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn random_in_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1., 1., 1.);
        if p.length_squared() >= 1.0 {
            return p;
        }
    }
}

fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - 2.0 * vector.dot(normal) * *normal
}
