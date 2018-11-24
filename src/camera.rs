use float::consts::PI;
use Float;

use rand::Rng;

use ray::Ray;
use vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Float,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: Float,
        aspect: Float,
        aperture: Float,
        focus_distance: Float,
    ) -> Camera {
        // The image plane is 1 unit away from the camera origin
        let theta = vertical_fov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            origin,
            lower_left_corner: origin
                - half_width * focus_distance * u
                - half_height * focus_distance * v
                - focus_distance * w,
            horizontal: 2. * half_width * focus_distance * u,
            vertical: 2. * half_height * focus_distance * v,
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
