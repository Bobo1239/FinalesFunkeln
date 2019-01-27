use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use image::{GenericImageView, ImageError};

use crate::math::float::Float;
use crate::math::{clamp, partial_min};
use crate::texture::Sample;
use crate::vec3::Vec3;

/// An image. The origin is at the upper-left corner.
#[derive(Debug, Clone)]
pub struct Image {
    image: Vec<Vec<Vec3>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            image: vec![vec![Vec3::default(); width]; height],
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Image, ImageError> {
        let image = image::open(path)?;
        let width = image.width();
        let height = image.height();
        let mut img = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let r = Float::from(pixel[0]) / 255.;
                let g = Float::from(pixel[1]) / 255.;
                let b = Float::from(pixel[2]) / 255.;
                row.push(Vec3::new(r, g, b));
            }
            img.push(row);
        }
        Ok(Image { image: img })
    }

    pub fn width(&self) -> usize {
        self.image[0].len()
    }

    pub fn height(&self) -> usize {
        self.image.len()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.image[y][x] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Vec3 {
        &self.image[y][x]
    }

    pub fn save_to_ppm(&self, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(&path)?;

        let header = format!("P6\n{}\n{}\n255\n", self.image[0].len(), self.image.len());
        file.write_all(header.as_bytes())?;

        for row in self.image.iter().rev() {
            for pixel in row {
                let r = (partial_min(pixel.r().sqrt(), 1.0) * 255.99) as u8;
                let g = (partial_min(pixel.g().sqrt(), 1.0) * 255.99) as u8;
                let b = (partial_min(pixel.b().sqrt(), 1.0) * 255.99) as u8;
                file.write_all(&[r, g, b])?;
            }
        }

        Ok(())
    }
}

impl Sample for Image {
    #[allow(clippy::many_single_char_names)]
    fn sample(&self, u: Float, v: Float, _: &Vec3) -> Vec3 {
        let i = clamp(u, 0.0, 1.0);
        let j = clamp(v, 0.0, 1.0);
        let mut x = (i * self.width() as Float).floor() as usize;
        let mut y = (j * self.height() as Float).floor() as usize;
        if x >= self.width() {
            x = self.width() - 1;
        }
        if y >= self.height() {
            y = self.height() - 1;
        }
        *self.get_pixel(x, self.height() - y - 1)
    }
}
