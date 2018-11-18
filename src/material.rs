use hit::HitRecord;
use ray::Ray;
use vec3::Vec3;

use rand::Rng;

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray, hit_record),
            Material::Metal(metal) => metal.scatter(ray, hit_record),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, hit_record),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered: Ray = Ray::new(hit_record.p, target - hit_record.p);
        Some((scattered, self.albedo))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        assert!(fuzz <= 1.);
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut refracted: Vec3 = Vec3::default();
        let reflect_prob: f32;
        let reflected = reflect(&r_in.direction(), &hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(&hit_record.normal) > 0.0
        {
            (
                -hit_record.normal,
                self.ref_idx,
                (self.ref_idx * r_in.direction().dot(&hit_record.normal)
                    / r_in.direction().length()),
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.ref_idx,
                (-r_in.direction().dot(&hit_record.normal) / r_in.direction().length()),
            )
        };
        let refr = refract(&r_in.direction(), &outward_normal, ni_over_nt);
        match refr {
            Some(refr) => {
                reflect_prob = schlick(cosine, self.ref_idx);
                refracted = refr
            }
            None => reflect_prob = 1.0,
        }
        if rand::thread_rng().gen::<f32>() < reflect_prob {
            Some((Ray::new(hit_record.p, reflected), attenuation))
        } else {
            Some((Ray::new(hit_record.p, refracted), attenuation))
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    (r0 + (1.0 - r0) * (1.0 - cosine).powi(5))
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1., 1., 1.);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - 2.0 * vector.dot(normal) * *normal
}
