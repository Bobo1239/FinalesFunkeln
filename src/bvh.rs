use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use crate::hit::{Hit, HitRecord};
use crate::math::float::{self, Float};
use crate::math::{partial_max, partial_min};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub enum BvhError {
    MissingBoundingBox,
    InvalidBoundingBox,
    TooFewElements(u8),
}

impl fmt::Display for BvhError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BvhError::MissingBoundingBox => write!(
                f,
                "Encountered object without bounding box during BVH construction"
            ),
            BvhError::InvalidBoundingBox => write!(
                f,
                "Encountered object with an invalid bounding box during BVH construction"
            ),
            BvhError::TooFewElements(n) => write!(
                f,
                "Bvh::new was called with {} objects but at least two objects are required",
                n
            ),
        }
    }
}

impl Error for BvhError {}

#[derive(Debug)]
pub struct Bvh {
    left: Box<dyn Hit>,
    right: Box<dyn Hit>,
    aabb: Aabb,
}

impl Bvh {
    pub fn new(
        mut hit_list: Vec<Box<dyn Hit>>,
        time_start: Float,
        time_end: Float,
    ) -> Result<Bvh, BvhError> {
        let mut rng = SmallRng::from_entropy();

        let axis = rng.gen_range(0, 3);
        let mut error = None;
        hit_list.sort_unstable_by(|a, b| {
            // The error handling is a bit awkward here because this is a closure.
            // We don't really care about performance issues here as the BVH will only be
            // constructed once anyways.
            let result = a
                .bounding_box(time_start, time_end)
                .ok_or(BvhError::MissingBoundingBox)
                .and_then(|a_aabb| {
                    Ok((
                        a_aabb,
                        b.bounding_box(time_start, time_end)
                            .ok_or(BvhError::MissingBoundingBox)?,
                    ))
                })
                .and_then(|(a_aabb, b_aabb)| {
                    a_aabb.min[axis]
                        .partial_cmp(&b_aabb.min[axis])
                        .ok_or(BvhError::InvalidBoundingBox)
                });
            match result {
                Ok(cmp) => cmp,
                Err(e) => {
                    if error.is_none() {
                        error = Some(e);
                    }
                    // Just return any Ordering as we'll exit anyways.
                    Ordering::Less
                }
            }
        });
        if let Some(error) = error {
            return Err(error);
        }

        let (left, right) = match hit_list.len() {
            0 => return Err(BvhError::TooFewElements(0)),
            1 => return Err(BvhError::TooFewElements(1)),
            2 => {
                let right = hit_list.pop().unwrap();
                let left = hit_list.pop().unwrap();
                (left, right)
            }
            3 => {
                let right = hit_list.pop().unwrap();
                let left = Box::new(Bvh::new(hit_list, time_start, time_end)?) as Box<dyn Hit>;
                (left, right)
            }
            _ => {
                let hit_list_len = hit_list.len(); // TODO: Not needed with NLL
                let right_half = hit_list.split_off(hit_list_len / 2);
                let left = Box::new(Bvh::new(hit_list, time_start, time_end)?) as Box<dyn Hit>;
                let right = Box::new(Bvh::new(right_half, time_start, time_end)?) as Box<dyn Hit>;
                (left, right)
            }
        };

        // The `unwrap()`s are safe as we'll only reach this part if all AABBs are valid.
        Ok(Bvh {
            aabb: left
                .bounding_box(time_start, time_end)
                .unwrap()
                .union(&right.bounding_box(time_start, time_end).unwrap()),
            left,
            right,
        })
    }
}

impl Hit for Bvh {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
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
