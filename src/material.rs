use ray::Ray;
use vec3::Vec3;

struct HitRecord;

trait Scatter {
	fn scatter(&self, ray: &Ray, attenuation: &Vec3, hit_record: &HitRecord) -> (Ray, f32);
}

struct Lambertian {
	albedo: f32,
}

impl Scatter for Lambertian {
	fn scatter(&self, ray: &Ray, attenuation: &Vec3, hit_record: &HitRecord) -> (Ray, f32) {

	}
}


fn random_in_sphere() -> Vec3{
	let mut p: Vec3;
	
}

// struct Metal


// impl Scatter for Metal 

// enum Material {
// 	Metal(Metal)
// }

// impl Scatter for Material