use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use rayon::prelude::*;

use finales_funkeln::bvh::{Bvh, BvhError};
use finales_funkeln::camera::{Camera, CameraParameters};
use finales_funkeln::hit::{FlipNormals, Hit};
use finales_funkeln::image::Image;
use finales_funkeln::material::*;
use finales_funkeln::math::float::{self, Float};
use finales_funkeln::ray::Ray;
use finales_funkeln::shape::*;
use finales_funkeln::texture::Texture;
use finales_funkeln::vec3::Vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height, samples_per_pixel) = if true {
        (1920, 1080, 1000)
    } else {
        (720, 480, 100)
    };
    let image = Arc::new(Mutex::new(Image::new(width, height)));

    let (hit_list, camera) = if false {
        let hit_list = vec![Box::new(random_scene(0.0, 1.0)?) as Box<dyn Hit>];
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
        (hit_list, camera)
    } else {
        let hit_list = cornell_box();
        let camera = {
            let origin = Vec3::new(278., 278., -800.);
            let look_at = Vec3::new(278., 278., 0.);
            let up = Vec3::new(0., 1., 0.);
            let time = 0.0;
            let parameters = CameraParameters {
                aspect_ratio: width as Float / height as Float,
                vertical_fov: 40.,
                focus_distance: 10.,
                aperture: 0.0,
                exposure_time: 1.0,
            };
            Camera::new(origin, look_at, up, parameters, time)
        };
        (hit_list, camera)
    };

    let progress_bar = ProgressBar::new(width as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {elapsed_precise}/{eta_precise} {wide_bar} {percent:3}%"),
    );
    progress_bar.enable_steady_tick(100);

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

fn color<T: Rng>(ray: &Ray, world: &[Box<dyn Hit>], depth: usize, rng: &mut T) -> Vec3 {
    // Set t_min to a value slightly above 0 to prevent "shadow acne"
    match world.hit(ray, 0.001, float::MAX) {
        None => Vec3::zero(),
        Some(hit_record) => {
            let emitted = hit_record
                .material
                .emit(hit_record.u, hit_record.v, &hit_record.p);
            match (
                depth < 50,
                hit_record.material.scatter(ray, &hit_record, rng),
            ) {
                (true, Some((scattered, attenuation))) => {
                    emitted + attenuation * color(&scattered, world, depth + 1, rng)
                }
                _ => emitted,
            }
        }
    }
}

fn random_scene(time_start: Float, time_end: Float) -> Result<Bvh, BvhError> {
    let mut rng = SmallRng::from_entropy();
    let mut list: Vec<Box<dyn Hit>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::lambertian(Texture::checker_board(
            Texture::constant(Vec3::new(0.2, 0.3, 0.1)),
            Texture::constant(Vec3::new(0.9, 0.9, 0.9)),
            0.1,
        )),
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
                    Material::lambertian(Texture::constant(Vec3::new(
                        rng.gen::<Float>() * rng.gen::<Float>(),
                        rng.gen::<Float>() * rng.gen::<Float>(),
                        rng.gen::<Float>() * rng.gen::<Float>(),
                    )))
                } else if choose_mat < 0.95 {
                    Material::metal(
                        Vec3::new(
                            0.5 * (1. + rng.gen::<Float>()),
                            0.5 * (1. + rng.gen::<Float>()),
                            0.5 * (1. + rng.gen::<Float>()),
                        ),
                        0.5 * rng.gen::<Float>(),
                    )
                } else {
                    Material::dielectric(1.5)
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
        Material::dielectric(1.5),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::lambertian(Texture::constant(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.),
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(-4., 3.5, 0.),
        1.,
        Material::diffuse_light(Texture::constant(Vec3::new(6., 6., 6.))),
    )));

    list.push(Box::new(XYRect::new(
        (3., 5.),
        (1., 5.),
        1.5,
        Material::diffuse_light(Texture::constant(Vec3::new(3., 3., 3.))),
    )));

    Bvh::new(list, time_start, time_end)
}

fn cornell_box() -> Vec<Box<dyn Hit>> {
    let mut vec: Vec<Box<dyn Hit>> = Vec::new();

    let red = Material::lambertian(Texture::constant(Vec3::new(0.65, 0.05, 0.05)));
    let white = Material::lambertian(Texture::constant(Vec3::new(0.73, 0.73, 0.73)));
    let green = Material::lambertian(Texture::constant(Vec3::new(0.12, 0.45, 0.15)));
    let light = Material::diffuse_light(Texture::constant(Vec3::new(15., 15., 15.)));

    const W: Float = 555.;

    vec.push(Box::new(FlipNormals(YZRect::new(
        (0., W),
        (0., W),
        W,
        green,
    ))));
    vec.push(Box::new(YZRect::new((0., W), (0., W), 0., red)));
    vec.push(Box::new(XZRect::new(
        (213., 343.),
        (227., 332.),
        W - 1.,
        light,
    )));
    vec.push(Box::new(FlipNormals(XZRect::new(
        (0., W),
        (0., W),
        W,
        white.clone(),
    ))));
    vec.push(Box::new(XZRect::new((0., W), (0., W), 0., white.clone())));
    vec.push(Box::new(FlipNormals(XYRect::new(
        (0., W),
        (0., W),
        W,
        white.clone(),
    ))));

    vec.push(Box::new(RectBox::new(
        Vec3::new(130., 0., 65.),
        Vec3::new(295., 165., 230.),
        white.clone(),
    )));
    vec.push(Box::new(RectBox::new(
        Vec3::new(265., 0., 295.),
        Vec3::new(430., 330., 460.),
        white.clone(),
    )));

    vec
}
