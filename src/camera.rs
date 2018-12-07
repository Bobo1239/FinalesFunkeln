use crate::math::float::consts::PI;
use crate::math::float::Float;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Rng;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Float,
    time: Float,
    parameters: CameraParameters,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraParameters {
    pub aspect_ratio: Float,
    pub vertical_fov: Float,
    pub focus_distance: Float,
    pub aperture: Float,
    pub exposure_time: Float,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        parameters: CameraParameters,
        time: Float,
    ) -> Camera {
        let p = parameters;

        // The image plane is 1 unit away from the camera origin
        let theta = p.vertical_fov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = p.aspect_ratio * half_height;

        let w = (origin - look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            origin,
            lower_left_corner: origin
                - half_width * p.focus_distance * u
                - half_height * p.focus_distance * v
                - p.focus_distance * w,
            horizontal: 2. * half_width * p.focus_distance * u,
            vertical: 2. * half_height * p.focus_distance * v,
            u,
            v,
            lens_radius: p.aperture / 2.,
            time,
            parameters: p,
        }
    }

    pub fn get_ray<T: Rng>(&self, s: Float, t: Float, rng: &mut T) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            self.time + rng.gen::<Float>() * self.parameters.exposure_time,
        )
    }
}

fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
