use math::float::Float;
use vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: Float,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: Float) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> Float {
        self.time
    }
}

impl Ray {
    pub fn point_at_parameter(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}
