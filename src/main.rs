#![cfg_attr(feature = "f64", allow(clippy::cast_lossless))]

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};
use rand::SeedableRng;
use rand::Rng as RandRng;
use rand_xoshiro::Xoshiro256StarStar;
use rayon::prelude::*;

use finales_funkeln::bvh::{Bvh, BvhError};
use finales_funkeln::camera::{Camera, CameraParameters};
use finales_funkeln::hit::Hit;
use finales_funkeln::image::Image;
use finales_funkeln::material::*;
use finales_funkeln::math::float::{self, Float};
use finales_funkeln::ray::Ray;
use finales_funkeln::shape::*;
use finales_funkeln::texture::Texture;
use finales_funkeln::vec3::Vec3;
use finales_funkeln::Rng;

type Prng = Xoshiro256StarStar;

// TODO: Revamp material/texture resource managment (Scene) so we don't need `Arc`s and can share
//       `Texture`s.

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height, samples_per_pixel) = if true {
        (1920, 1080, 1000)
    } else {
        (720, 480, 100)
    };
    let image = Arc::new(Mutex::new(Image::new(width, height)));

    let mut rng = Prng::from_entropy();
    let (hit_list, camera) = if false {
        let hit_list = if true {
            vec![Box::new(random_scene(0.0, 1.0, &mut rng)?) as Box<dyn Hit<Prng>>]
        } else {
            two_spheres()
        };
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
        let (hit_list, origin) = if true {
            (final_scene(0., 1., &mut rng), Vec3::new(478., 278., -600.))
        } else {
            (
                if true {
                    cornell_box()
                } else {
                    cornell_box_smoke()
                },
                Vec3::new(278., 278., -800.),
            )
        };
        let camera = {
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
        let mut rng = Prng::from_entropy();
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

fn color<R: Rng>(ray: &Ray, world: &[Box<dyn Hit<R>>], depth: usize, rng: &mut R) -> Vec3 {
    // Set t_min to a value slightly above 0 to prevent "shadow acne"
    match world.hit(ray, 0.001, float::MAX, rng) {
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

fn random_scene<R: Rng>(
    time_start: Float,
    time_end: Float,
    rng: &mut R,
) -> Result<Bvh<R>, BvhError> {
    let mut list: Vec<Box<dyn Hit<R>>> = Vec::new();

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

    Bvh::new(list, time_start, time_end, rng)
}

fn cornell_box<R: Rng>() -> Vec<Box<dyn Hit<R>>> {
    let mut vec: Vec<Box<dyn Hit<R>>> = Vec::new();

    let red = Material::lambertian(Texture::constant(Vec3::new(0.65, 0.05, 0.05)));
    let white = Material::lambertian(Texture::constant(Vec3::new(0.73, 0.73, 0.73)));
    let green = Material::lambertian(Texture::constant(Vec3::new(0.12, 0.45, 0.15)));
    let light = Material::diffuse_light(Texture::constant(Vec3::new(15., 15., 15.)));

    const W: Float = 555.;

    vec.push(Box::new(
        YZRect::new((0., W), (0., W), W, green).flip_normals(),
    ));
    vec.push(Box::new(YZRect::new((0., W), (0., W), 0., red)));
    vec.push(Box::new(XZRect::new(
        (213., 343.),
        (227., 332.),
        W - 1.,
        light,
    )));
    vec.push(Box::new(
        XZRect::new((0., W), (0., W), W, Arc::clone(&white)).flip_normals(),
    ));
    vec.push(Box::new(XZRect::new(
        (0., W),
        (0., W),
        0.,
        Arc::clone(&white),
    )));
    vec.push(Box::new(
        XYRect::new((0., W), (0., W), W, Arc::clone(&white)).flip_normals(),
    ));

    vec.push(Box::new(
        RectBox::new(
            Vec3::zero(),
            Vec3::new(165., 165., 165.),
            Arc::clone(&white),
        )
        .rotate_y(-18.)
        .translate(Vec3::new(130., 0., 65.)),
    ));
    vec.push(Box::new(
        RectBox::new(
            Vec3::zero(),
            Vec3::new(165., 330., 165.),
            Arc::clone(&white),
        )
        .rotate_y(15.)
        .translate(Vec3::new(265., 0., 295.)),
    ));

    vec
}

fn cornell_box_smoke<R: Rng>() -> Vec<Box<dyn Hit<R>>> {
    let mut vec: Vec<Box<dyn Hit<R>>> = Vec::new();

    let red = Material::lambertian(Texture::constant(Vec3::new(0.65, 0.05, 0.05)));
    let white = Material::lambertian(Texture::constant(Vec3::new(0.73, 0.73, 0.73)));
    let green = Material::lambertian(Texture::constant(Vec3::new(0.12, 0.45, 0.15)));
    let light = Material::diffuse_light(Texture::constant(Vec3::new(7., 7., 7.)));

    const W: Float = 555.;

    vec.push(Box::new(
        YZRect::new((0., W), (0., W), W, green).flip_normals(),
    ));
    vec.push(Box::new(YZRect::new((0., W), (0., W), 0., red)));
    vec.push(Box::new(XZRect::new(
        (213., 343.),
        (227., 332.),
        W - 1.,
        light,
    )));
    vec.push(Box::new(
        XZRect::new((0., W), (0., W), W, Arc::clone(&white)).flip_normals(),
    ));
    vec.push(Box::new(XZRect::new(
        (0., W),
        (0., W),
        0.,
        Arc::clone(&white),
    )));
    vec.push(Box::new(
        XYRect::new((0., W), (0., W), W, Arc::clone(&white)).flip_normals(),
    ));

    let box1 = RectBox::new(
        Vec3::zero(),
        Vec3::new(165., 165., 165.),
        Arc::clone(&white),
    )
    .rotate_y(-18.)
    .translate(Vec3::new(130., 0., 65.));

    let box2 = RectBox::new(
        Vec3::zero(),
        Vec3::new(165., 330., 165.),
        Arc::clone(&white),
    )
    .rotate_y(15.)
    .translate(Vec3::new(265., 0., 295.));

    vec.push(Box::new(ConstantMedium::new(
        box1,
        0.01,
        Texture::constant(Vec3::new(1.0, 1.0, 1.0)),
    )));
    vec.push(Box::new(ConstantMedium::new(
        box2,
        0.01,
        Texture::constant(Vec3::new(0.0, 0.0, 0.0)),
    )));

    vec
}

fn two_spheres<R: Rng>() -> Vec<Box<dyn Hit<R>>> {
    let mut vec: Vec<Box<dyn Hit<R>>> = Vec::new();

    let image = Image::load_from_file("world.topo.200405.3x5400x2700.jpg").unwrap();
    let light = Material::diffuse_light(Texture::constant(Vec3::new(1., 1., 1.)));

    vec.push(Box::new(XZRect::new(
        (-100., 100.),
        (-100., 100.),
        150.,
        light,
    )));
    vec.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::lambertian(Texture::noise(4.)),
    )));
    vec.push(Box::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Material::lambertian(Texture::image(image)),
    )));

    vec
}

fn final_scene<R: Rng>(time_start: Float, time_end: Float, rng: &mut R) -> Vec<Box<dyn Hit<R>>> {
    let mut vec: Vec<Box<dyn Hit<R>>> = Vec::new();

    let white = Material::lambertian(Texture::constant(Vec3::new(0.73, 0.73, 0.73)));
    let ground_material = Material::lambertian(Texture::constant(Vec3::new(0.48, 0.83, 0.53)));
    let light = Material::diffuse_light(Texture::constant(Vec3::new(7., 7., 7.)));

    let mut ground: Vec<Box<dyn Hit<R>>> = Vec::new();
    let n = 20;
    for i in 0..n {
        for j in 0..n {
            let w = 100.;
            let x0 = -1000. + i as Float * w;
            let z0 = -1000. + j as Float * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = 100. * (rng.gen::<Float>() + 0.01);
            let z1 = z0 + w;
            ground.push(Box::new(RectBox::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground_material.clone(),
            )));
        }
    }
    vec.push(Box::new(
        Bvh::new(ground, time_start, time_end, rng).unwrap(),
    ));

    vec.push(Box::new(XZRect::new(
        (123., 423.),
        (147., 412.),
        553.,
        light,
    )));

    vec.push(Box::new(Sphere::new_moving(
        Vec3::new(400., 400., 200.),
        50.,
        Material::lambertian(Texture::constant(Vec3::new(0.7, 0.3, 0.1))),
        Vec3::new(30., 0., 0.),
    )));

    vec.push(Box::new(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        Material::dielectric(1.5),
    )));

    vec.push(Box::new(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Material::metal(Vec3::new(0.8, 0.8, 0.9), 1.),
    )));

    // TODO: Our Rust version currently needs to clone the inner hit object of a ConstantMedium
    //       while the C version just stores a pointer to it. Probably related to the resource
    //       managment TODO.
    let subsurface = Sphere::new(Vec3::new(360., 150., 145.), 70., Material::dielectric(1.5));
    vec.push(Box::new(subsurface.clone()));
    vec.push(Box::new(ConstantMedium::new(
        subsurface,
        0.2,
        Texture::constant(Vec3::new(0.2, 0.4, 0.9)),
    )));

    vec.push(Box::new(ConstantMedium::new(
        Sphere::new(Vec3::zero(), 5000., Material::dielectric(1.5)),
        0.0001,
        Texture::constant(Vec3::new(1., 1., 1.)),
    )));

    vec.push(Box::new(Sphere::new(
        Vec3::new(400., 200., 400.),
        100.,
        Material::lambertian(Texture::image(
            Image::load_from_file("world.topo.200405.3x5400x2700.jpg").unwrap(),
        )),
    )));

    vec.push(Box::new(Sphere::new(
        Vec3::new(220., 280., 300.),
        80.,
        Material::lambertian(Texture::noise(0.1)),
    )));

    let mut box_list: Vec<Box<dyn Hit<R>>> = Vec::new();
    for _ in 0..1000 {
        box_list.push(Box::new(Sphere::new(
            Vec3::new(
                165. * rng.gen::<Float>(),
                165. * rng.gen::<Float>(),
                165. * rng.gen::<Float>(),
            ),
            10.,
            white.clone(),
        )));
    }
    vec.push(Box::new(
        Bvh::new(box_list, time_start, time_end, rng)
            .unwrap()
            .rotate_y(15.)
            .translate(Vec3::new(-100., 270., 395.)),
    ));

    vec
}
