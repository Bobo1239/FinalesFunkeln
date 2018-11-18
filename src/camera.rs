use std::f32::consts::PI;

use ray::Ray;
use vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, vertical_fov: f32, aspect: f32) -> Camera {
        // The image plane is 1 unit away from the camera origin
        let theta = vertical_fov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2. * half_width * u,
            vertical: 2. * half_height * v,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
