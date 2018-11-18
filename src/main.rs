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

fn main() -> Result<(), Box<Error>> {
    let width = 200;
    let height = 100;
    let samples_per_pixel = 10;
    let mut image = Image::new(width, height);
    let mut rng = rand::thread_rng();

    let camera = {
        let origin = Vec3::new(13., 2., 3.);
        let look_at = Vec3::new(0., 0., 0.);
        let up = Vec3::new(0., 1., 0.);
        let vertical_fov = 20.;
        let aspect_ratio = width as f32 / height as f32;
        let aperture = 0.1;
        let focus_distance = 10.;
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
    let hit_list = random_scene();

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
        println!("{} %", (x + 1) as f32 / width as f32 * 100.);
    }

    image.save_to_ppm(Path::new("out.ppm"))?;

    Ok(())
}

fn color(ray: &Ray, world: &[Box<Hit>], depth: usize) -> Vec3 {
    match world.hit(ray, 0.0, f32::MAX) {
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
        Some(hit_record) => match (depth < 50, hit_record.material.scatter(ray, &hit_record)) {
            (true, Some((scattered, attenuation))) => {
                attenuation * color(&scattered, world, depth + 1)
            }
            _ => Vec3::new(0., 0., 0.),
        },
    }
}

fn random_scene() -> Vec<Box<Hit>> {
    let mut rng = rand::thread_rng();
    let mut list: Vec<Box<Hit>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    Material::Lambertian(Lambertian::new(Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    )))
                } else if choose_mat < 0.95 {
                    Material::Metal(Metal::new(
                        Vec3::new(
                            0.5 * (1. + rng.gen::<f32>()),
                            0.5 * (1. + rng.gen::<f32>()),
                            0.5 * (1. + rng.gen::<f32>()),
                        ),
                        0.5 * rng.gen::<f32>(),
                    ))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };
                list.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Dielectric(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));

    list
}
