use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use hit::{Hit, HitRecord};
use math::float::{self, Float};
use math::{partial_max, partial_min};
use ray::Ray;
use vec3::Vec3;

#[derive(Debug)]
pub struct Bvh {
    left: Box<Hit>,
    right: Box<Hit>,
    aabb: Aabb,
}

impl Bvh {
    pub fn new(mut hit_list: Vec<Box<Hit>>, time_start: Float, time_end: Float) -> Bvh {
        // TODO: Too many panic!()s in here....
        let mut rng = SmallRng::from_entropy();

        let axis = rng.gen_range(0, 3);
        hit_list.sort_unstable_by(|a, b| {
            let a_aabb = a
                .bounding_box(time_start, time_end)
                .expect("object without bounding box in Bvh::new");
            let b_aabb = b
                .bounding_box(time_start, time_end)
                .expect("object without bounding box in Bvh::new");
            a_aabb.min[axis]
                .partial_cmp(&b_aabb.min[axis])
                .expect("partial_cmp failed in Bvh::new")
        });

        let (left, right) = match hit_list.len() {
            0 => panic!("empty hit_list in Bvh::new"),
            1 => panic!("single element hit_list in Bvh::new"),
            2 => {
                let right = hit_list.pop().unwrap();
                let left = hit_list.pop().unwrap();
                (left, right)
            }
            3 => {
                let right = hit_list.pop().unwrap();
                let left = Box::new(Bvh::new(hit_list, time_start, time_end)) as Box<Hit>;
                (left, right)
            }
            _ => {
                let hit_list_len = hit_list.len(); // TODO: Not needed after NLL
                let right_half = hit_list.split_off(hit_list_len / 2);
                let left = Box::new(Bvh::new(hit_list, time_start, time_end)) as Box<Hit>;
                let right = Box::new(Bvh::new(right_half, time_start, time_end)) as Box<Hit>;
                (left, right)
            }
        };

        Bvh {
            aabb: left
                .bounding_box(time_start, time_end)
                .unwrap()
                .union(&right.bounding_box(time_start, time_end).unwrap()),
            left,
            right,
        }
    }
}

impl Hit for Bvh {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            match (
                self.left.hit(ray, t_min, t_max),
                self.right.hit(ray, t_min, t_max),
            ) {
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
                (Some(h), None) | (None, Some(h)) => Some(h),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: Float, _: Float) -> Option<Aabb> {
        Some(self.aabb)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn empty() -> Aabb {
        Aabb {
            min: Vec3::new(float::MAX, float::MAX, float::MAX),
            max: Vec3::new(float::MIN, float::MIN, float::MIN),
        }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: Float, mut t_max: Float) -> bool {
        // Slab method to compute whether the ray intersects with the AABB.
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            }
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        Aabb::new(
            Vec3::new(
                partial_min(self.min.x(), other.min.x()),
                partial_min(self.min.y(), other.min.y()),
                partial_min(self.min.z(), other.min.z()),
            ),
            Vec3::new(
                partial_max(self.max.x(), other.max.x()),
                partial_max(self.max.y(), other.max.y()),
                partial_max(self.max.z(), other.max.z()),
            ),
        )
    }
}
