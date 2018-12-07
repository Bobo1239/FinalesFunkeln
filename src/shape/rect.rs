use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::bvh::Aabb;
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::math::float::Float;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct XYRect(GenericRect<XY>);

impl XYRect {
    pub fn new(x: (Float, Float), y: (Float, Float), z: Float, material: Arc<Material>) -> XYRect {
        XYRect(GenericRect::new(x, y, z, material))
    }
}

impl Hit for XYRect {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        self.0.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.0.bounding_box(time_start, time_end)
    }
}

#[derive(Debug)]
pub struct YZRect(GenericRect<YZ>);

impl YZRect {
    pub fn new(x: (Float, Float), y: (Float, Float), z: Float, material: Arc<Material>) -> YZRect {
        YZRect(GenericRect::new(x, y, z, material))
    }
}

impl Hit for YZRect {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        self.0.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.0.bounding_box(time_start, time_end)
    }
}

#[derive(Debug)]
pub struct XZRect(GenericRect<XZ>);

impl XZRect {
    pub fn new(x: (Float, Float), y: (Float, Float), z: Float, material: Arc<Material>) -> XZRect {
        XZRect(GenericRect::new(x, y, z, material))
    }
}

impl Hit for XZRect {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        self.0.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, time_start: Float, time_end: Float) -> Option<Aabb> {
        self.0.bounding_box(time_start, time_end)
    }
}

#[derive(Debug)]
struct GenericRect<A: Axis> {
    pub a: (Float, Float),
    pub b: (Float, Float),
    pub c: Float,
    material: Arc<Material>,
    axis: PhantomData<A>,
}

impl<A: Axis> GenericRect<A> {
    pub fn new(
        a: (Float, Float),
        b: (Float, Float),
        c: Float,
        material: Arc<Material>,
    ) -> GenericRect<A> {
        GenericRect {
            a,
            b,
            c,
            material,
            axis: PhantomData,
        }
    }
}

impl<A: Axis> Hit for GenericRect<A> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord<'_>> {
        let t = (self.c - A::c(&ray.origin())) / A::c(&ray.direction());
        if t < t_min || t > t_max {
            return None;
        }
        let a = A::a(&ray.origin()) + t * A::a(&ray.direction());
        let b = A::b(&ray.origin()) + t * A::b(&ray.direction());
        if a < self.a.0 || a > self.a.1 || b < self.b.0 || b > self.b.1 {
            None
        } else {
            Some(HitRecord {
                t,
                u: (a - self.a.0) / (self.a.1 - self.a.0),
                v: (b - self.b.0) / (self.b.1 - self.b.0),
                p: ray.point_at_parameter(t),
                normal: A::normal(),
                material: &self.material,
            })
        }
    }

    fn bounding_box(&self, _: Float, _: Float) -> Option<Aabb> {
        Some(A::bounding_box(self.a, self.b, self.c))
    }
}

trait Axis: Debug + Sync + Send {
    fn a(vec3: &Vec3) -> Float;
    fn b(vec3: &Vec3) -> Float;
    fn c(vec3: &Vec3) -> Float;
    fn normal() -> Vec3;
    fn bounding_box(a: (Float, Float), b: (Float, Float), c: Float) -> Aabb;
}

#[derive(Debug)]
struct XY;

impl Axis for XY {
    fn a(vec3: &Vec3) -> Float {
        vec3.x()
    }
    fn b(vec3: &Vec3) -> Float {
        vec3.y()
    }
    fn c(vec3: &Vec3) -> Float {
        vec3.z()
    }
    fn normal() -> Vec3 {
        Vec3::new(0., 0., 1.)
    }
    fn bounding_box(a: (Float, Float), b: (Float, Float), c: Float) -> Aabb {
        Aabb::new(
            Vec3::new(a.0, b.0, c - 0.0001),
            Vec3::new(a.1, b.1, c + 0.0001),
        )
    }
}

#[derive(Debug)]
struct YZ;

impl Axis for YZ {
    fn a(vec3: &Vec3) -> Float {
        vec3.y()
    }
    fn b(vec3: &Vec3) -> Float {
        vec3.z()
    }
    fn c(vec3: &Vec3) -> Float {
        vec3.x()
    }
    fn normal() -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
    fn bounding_box(a: (Float, Float), b: (Float, Float), c: Float) -> Aabb {
        Aabb::new(
            Vec3::new(c - 0.0001, a.0, b.0),
            Vec3::new(c + 0.0001, a.1, b.1),
        )
    }
}

#[derive(Debug)]
struct XZ;

impl Axis for XZ {
    fn a(vec3: &Vec3) -> Float {
        vec3.x()
    }
    fn b(vec3: &Vec3) -> Float {
        vec3.z()
    }
    fn c(vec3: &Vec3) -> Float {
        vec3.y()
    }
    fn normal() -> Vec3 {
        Vec3::new(0., 1., 0.)
    }
    fn bounding_box(a: (Float, Float), b: (Float, Float), c: Float) -> Aabb {
        Aabb::new(
            Vec3::new(a.0, c - 0.0001, b.0),
            Vec3::new(a.1, c + 0.0001, b.1),
        )
    }
}
