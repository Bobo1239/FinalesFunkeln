use vec3::Vec3;
use Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

impl Ray {
    pub fn point_at_parameter(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}
