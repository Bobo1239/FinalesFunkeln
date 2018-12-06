use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::math::partial_min;
use crate::vec3::Vec3;

/// An image. The origin is at the upper-left corner.
#[derive(Debug)]
pub struct Image {
    image: Vec<Vec<Vec3>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            image: vec![vec![Vec3::default(); width]; height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.image[y][x] = color;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> &Vec3 {
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
