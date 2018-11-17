extern crate finales_funkeln;

use std::error::Error;
use std::path::Path;

use finales_funkeln::image::Image;
use finales_funkeln::ray::Ray;
use finales_funkeln::vec3::Vec3;

fn background_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<Error>> {
    let width = 800;
    let height = 400;
    let mut image = Image::new(width, height);

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    for x in 0..width {
        for y in 0..height {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = background_color(&ray);

            image.set_pixel(x, y, color);
        }
    }

    image.save_to_ppm(Path::new("out.ppm"))?;

    Ok(())
}
