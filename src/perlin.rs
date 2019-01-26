use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::math::float::Float;
use crate::texture::Sample;
use crate::vec3::Vec3;

const N: usize = 256;

lazy_static! {
    static ref RND_VEC3: [Vec3; N] = {
        let mut rng = rand::thread_rng();
        let mut rnd_vec3 = [Vec3::default(); N];
        for v in rnd_vec3.iter_mut() {
            *v = Vec3::new(
                -1. + 2. * rng.gen::<Float>(),
                -1. + 2. * rng.gen::<Float>(),
                -1. + 2. * rng.gen::<Float>(),
            )
            .unit_vector();
        }
        rnd_vec3
    };
    static ref PERM_X: [usize; N] = {
        let mut perm = [0; N];
        for (i, v) in perm.iter_mut().enumerate() {
            *v = i;
        }
        perm.shuffle(&mut rand::thread_rng());
        perm
    };
    static ref PERM_Y: [usize; N] = {
        let mut perm = *PERM_X;
        perm.shuffle(&mut rand::thread_rng());
        perm
    };
    static ref PERM_Z: [usize; N] = {
        let mut perm = *PERM_X;
        perm.shuffle(&mut rand::thread_rng());
        perm
    };
}

#[allow(clippy::many_single_char_names)]
fn noise(p: &Vec3) -> Float {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as usize;
    let j = p.y().floor() as usize;
    let k = p.z().floor() as usize;

    let mut c = [[[Vec3::default(); 2]; 2]; 2];
    for (di, c) in c.iter_mut().enumerate() {
        for (dj, c) in c.iter_mut().enumerate() {
            for (dk, c) in c.iter_mut().enumerate() {
                *c = RND_VEC3
                    [PERM_X[(i + di) & 255] ^ PERM_Y[(j + dj) & 255] ^ PERM_Z[(k + dk) & 255]];
            }
        }
    }

    interpolate_perlin(&c, u, v, w)
}

#[allow(clippy::many_single_char_names)]
fn interpolate_perlin(c: &[[[Vec3; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    let mut accum = 0.0;
    for (i, c) in c.iter().enumerate() {
        for (j, c) in c.iter().enumerate() {
            for (k, c) in c.iter().enumerate() {
                let i = i as Float;
                let j = j as Float;
                let k = k as Float;

                let weight = Vec3::new(u - i, v - j, w - k);
                accum += (i * uu + (1. - i) * (1. - uu))
                    * (j * vv + (1. - j) * (1. - vv))
                    * (k * ww + (1. - k) * (1. - ww))
                    * c.dot(&weight);
            }
        }
    }
    accum
}

fn turbulence(p: &Vec3, depth: u8) -> Float {
    let mut accum = 0.0;
    let mut temp_p = *p;
    let mut weight = 1.0;
    for _ in 0..depth {
        accum += weight * noise(&temp_p);
        weight *= 0.5;
        temp_p *= 2.;
    }
    accum.abs()
}

#[derive(Debug, Clone)]
pub struct Perlin {
    scale: Float,
}

impl Perlin {
    pub fn new(scale: Float) -> Perlin {
        Perlin { scale }
    }
}

impl Sample for Perlin {
    fn sample(&self, _: Float, _: Float, p: &Vec3) -> Vec3 {
        Vec3::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.x() + 5. * turbulence(&(self.scale * *p), 7)).sin())
    }
}
