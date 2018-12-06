use crate::hit::HitRecord;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::texture::Sample;
use crate::texture::Texture;
use crate::vec3::Vec3;

use rand::Rng;

pub trait MaterialTrait {
    fn scatter<T: Rng>(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, Vec3)>;

    #[allow(unused_variables)]
    fn emit(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Material {
    pub fn lambertian(texture: Texture) -> Material {
        Material::Lambertian(Lambertian::new(texture))
    }

    pub fn metal(albedo: Vec3, fuzz: Float) -> Material {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(ref_idx: Float) -> Material {
        Material::Dielectric(Dielectric::new(ref_idx))
    }

    pub fn diffuse_light(texture: Texture) -> Material {
        Material::DiffuseLight(DiffuseLight::new(texture))
    }
}

impl MaterialTrait for Material {
    fn scatter<T: Rng>(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray, hit_record, rng),
            Material::Metal(metal) => metal.scatter(ray, hit_record, rng),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, hit_record, rng),
            Material::DiffuseLight(diffuse_light) => diffuse_light.scatter(ray, hit_record, rng),
        }
    }

    fn emit(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        match self {
            Material::Lambertian(lambertian) => lambertian.emit(u, v, p),
            Material::Metal(metal) => metal.emit(u, v, p),
            Material::Dielectric(dielectric) => dielectric.emit(u, v, p),
            Material::DiffuseLight(diffuse_light) => diffuse_light.emit(u, v, p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Lambertian {
        Lambertian { texture }
    }
}

impl MaterialTrait for Lambertian {
    fn scatter<T: Rng>(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, Vec3)> {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_unit_sphere(rng);
        let scattered: Ray = Ray::new(hit_record.p, target - hit_record.p, ray.time());
        Some((scattered, self.texture.sample(0., 0., &hit_record.p)))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Float) -> Metal {
        assert!(fuzz <= 1.);
        Metal { albedo, fuzz }
    }
}

impl MaterialTrait for Metal {
    fn scatter<T: Rng>(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(rng),
            ray.time(),
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
    ref_idx: Float,
}

impl Dielectric {
    pub fn new(ref_idx: Float) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl MaterialTrait for Dielectric {
    fn scatter<T: Rng>(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut T,
    ) -> Option<(Ray, Vec3)> {
        let mut refracted: Vec3 = Vec3::zero();
        let reflect_prob: Float;
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
        let direction = if rng.gen::<Float>() < reflect_prob {
            reflected
        } else {
            refracted
        };
        Some((Ray::new(hit_record.p, direction, r_in.time()), attenuation))
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    texture: Texture,
}

impl DiffuseLight {
    pub fn new(texture: Texture) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl MaterialTrait for DiffuseLight {
    fn scatter<T: Rng>(&self, _: &Ray, _: &HitRecord, _: &mut T) -> Option<(Ray, Vec3)> {
        None
    }

    fn emit(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        self.texture.sample(u, v, p)
    }
}

fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    (r0 + (1.0 - r0) * (1.0 - cosine).powi(5))
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: Float) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
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
