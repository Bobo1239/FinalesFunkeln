use crate::image::Image;
use crate::math::float::Float;
use crate::perlin::Perlin;
use crate::vec3::Vec3;

pub trait Sample {
    fn sample(&self, u: Float, v: Float, p: &Vec3) -> Vec3;
}

#[derive(Debug, Clone)]
pub enum Texture {
    Constant(Constant),
    CheckerBoard(CheckerBoard),
    Noise(Perlin),
    Image(Image),
}

impl Texture {
    pub fn constant(color: Vec3) -> Texture {
        Texture::Constant(Constant { color })
    }

    pub fn checker_board(texture0: Texture, texture1: Texture, square_size: Float) -> Texture {
        Texture::CheckerBoard(CheckerBoard {
            texture0: Box::new(texture0),
            texture1: Box::new(texture1),
            multiplier: 1. / square_size,
        })
    }

    pub fn noise(scale: Float) -> Texture {
        Texture::Noise(Perlin::new(scale))
    }

    pub fn image(image: Image) -> Texture {
        Texture::Image(image)
    }
}

impl Sample for Texture {
    fn sample(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(t) => t.sample(u, v, p),
            Texture::CheckerBoard(t) => t.sample(u, v, p),
            Texture::Noise(t) => t.sample(u, v, p),
            Texture::Image(t) => t.sample(u, v, p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constant {
    color: Vec3,
}

impl Sample for Constant {
    fn sample(&self, _: Float, _: Float, _: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct CheckerBoard {
    texture0: Box<Texture>,
    texture1: Box<Texture>,
    multiplier: Float,
}

impl Sample for CheckerBoard {
    fn sample(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        let sines = (self.multiplier * p.x()).sin()
            * (self.multiplier * p.y()).sin()
            * (self.multiplier * p.z()).sin();
        if sines < 0.0 {
            self.texture0.sample(u, v, p)
        } else {
            self.texture1.sample(u, v, p)
        }
    }
}
