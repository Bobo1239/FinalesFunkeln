use hit::HitRecord;
use ray::Ray;
use vec3::Vec3;

use rand::Rng;

trait Scatter {
    fn scatter(&self, ray: &Ray, attenuation: &Vec3, hit_record: &HitRecord) -> (Ray, Vec3);
}

struct Lambertian {
    albedo: Vec3,
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, _attenuation: &Vec3, hit_record: &HitRecord) -> (Ray, Vec3) {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_sphere();
        let scattered: Ray = Ray::new(hit_record.p, target - hit_record.p);
        (scattered, self.albedo)
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

// struct Metal

// impl Scatter for Metal

// enum Material {
// 	Metal(Metal)
// }

// impl Scatter for Material
