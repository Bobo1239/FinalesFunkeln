use material::Material;
use ray::Ray;
use vec3::Vec3;
use Float;

pub trait Hit: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl Hit for [Box<Hit>] {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.iter()
            .fold((None, t_max), |(closest_hit, closest_t), item| {
                match item.hit(ray, t_min, closest_t) {
                    Some(hit_record) => {
                        let t = hit_record.t;
                        (Some(hit_record), t)
                    }
                    None => (closest_hit, closest_t),
                }
            })
            .0
    }
}
