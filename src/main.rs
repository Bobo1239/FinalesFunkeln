extern crate finales_funkeln;
extern crate rand;

use std::error::Error;
use std::f32;
use std::path::Path;

use rand::Rng;

use finales_funkeln::camera::Camera;
use finales_funkeln::hit::Hit;
use finales_funkeln::image::Image;
use finales_funkeln::material::*;
use finales_funkeln::ray::Ray;
use finales_funkeln::sphere::Sphere;
use finales_funkeln::vec3::Vec3;

fn color(ray: &Ray, world: &[Box<Hit>], depth: usize) -> Vec3 {
    match world.hit(ray, 0.0, f32::MAX) {
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
        Some(hit_record) => match hit_record.material.scatter(ray, &hit_record) {
            Some((scattered, attenuation)) => attenuation * color(&scattered, world, depth + 1),
            None => Vec3::new(0., 0., 0.),
        },
    }
}

fn main() -> Result<(), Box<Error>> {
    let width = 800;
    let height = 400;
    let samples_per_pixel = 100;
    let mut image = Image::new(width, height);
    let camera = {
        let origin = Vec3::new(0., 2.0, 2.0);
        let look_at = Vec3::new(0., 0., -1.);
        let up = Vec3::new(0., 1., 0.);
        let vertical_fov = 20.;
        let aspect_ratio = 800 as f32 / 400 as f32;
        let aperture = 2.0;
        let focus_distance = (look_at - origin).length();
        Camera::new(
            origin,
            look_at,
            up,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_distance,
        )
    };
    let mut rng = rand::thread_rng();

    let mut hit_list: Vec<Box<Hit>> = Vec::new();
    hit_list.push(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Material::Lambertian(Lambertian::new(Vec3::new(1., 1., 1.))),
    )));
    hit_list.push(Box::new(Sphere::new(
        Vec3::new(0.6, 0., -0.5),
        0.25,
        Material::Lambertian(Lambertian::new(Vec3::new(1., 0., 0.))),
    )));
    hit_list.push(Box::new(Sphere::new(
        Vec3::new(-0.6, 0., -0.5),
        0.25,
        Material::Metal(Metal::new(Vec3::new(0., 0., 1.))),
    )));
    hit_list.push(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Material::Lambertian(Lambertian::new(Vec3::new(0., 0.2, 0.))),
    )));

    for x in 0..width {
        for y in 0..height {
            let mut color_acc = Vec3::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let s = (x as f32 + rng.gen::<f32>()) / width as f32;
                let t = (y as f32 + rng.gen::<f32>()) / height as f32;

                let ray = camera.get_ray(s, t);
                color_acc += color(&ray, &hit_list, 0);
            }
            image.set_pixel(x, y, color_acc / samples_per_pixel as f32);
        }
    }

    image.save_to_ppm(Path::new("out.ppm"))?;

    Ok(())
}
