extern crate finales_funkeln;
extern crate indicatif;
extern crate rand;
extern crate rayon;

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use rayon::prelude::*;

use finales_funkeln::camera::{Camera, CameraParameters};
use finales_funkeln::float;
use finales_funkeln::hit::Hit;
use finales_funkeln::image::Image;
use finales_funkeln::material::*;
use finales_funkeln::ray::Ray;
use finales_funkeln::sphere::Sphere;
use finales_funkeln::vec3::Vec3;
use finales_funkeln::Float;

fn main() -> Result<(), Box<Error>> {
    let width = 1920;
    let height = 1080;
    let samples_per_pixel = 1000;
    let image = Arc::new(Mutex::new(Image::new(width, height)));

    let camera = {
        let origin = Vec3::new(13., 2., 3.);
        let look_at = Vec3::new(0., 0., 0.);
        let up = Vec3::new(0., 1., 0.);
        let time = 0.0;
        let parameters = CameraParameters {
            aspect_ratio: width as Float / height as Float,
            vertical_fov: 20.,
            focus_distance: 10.,
            aperture: 0.1,
            exposure_time: 1.0,
        };
        Camera::new(origin, look_at, up, parameters, time)
    };
    let hit_list = random_scene();

    let progress_bar = ProgressBar::new(width as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {elapsed_precise}/{eta_precise} {wide_bar} {percent:3}%"),
    );
    progress_bar.enable_steady_tick(100);
    // Tick once so the spinner starts (otherwise it doesn't move until the first state update)
    // See: https://github.com/mitsuhiko/indicatif/issues/36
    progress_bar.tick();

    (0..width).into_par_iter().for_each(|x| {
        // TODO: Use xoshiro256** once https://github.com/rust-random/rand/pull/642
        //       is in rand release.
        let mut rng = SmallRng::from_entropy();
        let mut column = Vec::with_capacity(height);
        for y in 0..height {
            let mut color_acc = Vec3::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let s = (x as Float + rng.gen::<Float>()) / width as Float;
                let t = (y as Float + rng.gen::<Float>()) / height as Float;

                let ray = camera.get_ray(s, t, &mut rng);
                color_acc += color(&ray, &hit_list, 0, &mut rng);
            }
            column.push(color_acc / samples_per_pixel as Float);
        }
        let mut image = image.lock().unwrap();
        for (y, p) in column.iter().enumerate() {
            image.set_pixel(x, y, *p);
        }
        progress_bar.inc(1);
    });
    progress_bar.finish();

    image.lock().unwrap().save_to_ppm(Path::new("out.ppm"))?;

    Ok(())
}

fn color<T: Rng>(ray: &Ray, world: &[Box<Hit>], depth: usize, rng: &mut T) -> Vec3 {
    // Set t_min to a value slight above 0 to prevent "shadow acne"
    match world.hit(ray, 0.001, float::MAX) {
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
        Some(hit_record) => match (
            depth < 50,
            hit_record.material.scatter(ray, &hit_record, rng),
        ) {
            (true, Some((scattered, attenuation))) => {
                attenuation * color(&scattered, world, depth + 1, rng)
            }
            _ => Vec3::new(0., 0., 0.),
        },
    }
}

fn random_scene() -> Vec<Box<Hit>> {
    let mut rng = SmallRng::from_entropy();
    let mut list: Vec<Box<Hit>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: Float = rng.gen();
            let center = Vec3::new(
                a as Float + 0.9 * rng.gen::<Float>(),
                0.2,
                b as Float + 0.9 * rng.gen::<Float>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    Material::Lambertian(Lambertian::new(Vec3::new(
                        rng.gen::<Float>() * rng.gen::<Float>(),
                        rng.gen::<Float>() * rng.gen::<Float>(),
                        rng.gen::<Float>() * rng.gen::<Float>(),
                    )))
                } else if choose_mat < 0.95 {
                    Material::Metal(Metal::new(
                        Vec3::new(
                            0.5 * (1. + rng.gen::<Float>()),
                            0.5 * (1. + rng.gen::<Float>()),
                            0.5 * (1. + rng.gen::<Float>()),
                        ),
                        0.5 * rng.gen::<Float>(),
                    ))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };
                list.push(Box::new(Sphere::new_moving(
                    center,
                    0.2,
                    material,
                    Vec3::new(0., 0.5 * rng.gen::<Float>(), 0.),
                )));
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
        Vec3::new(4., 1., 0.),
        1.,
        Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));

    list
}
