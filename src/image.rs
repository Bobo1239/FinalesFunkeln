use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use vec3::Vec3;

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

        let header = format!("P3\n{}\n{}\n255\n", self.image[0].len(), self.image.len());
        file.write_all(header.as_bytes())?;

        for row in &self.image {
            for pixel in row {
                let r = (pixel.r() * 255.) as u8;
                let g = (pixel.g() * 255.) as u8;
                let b = (pixel.b() * 255.) as u8;
                let pixel_val = format!("{} {} {}\n", r, g, b);
                file.write_all(pixel_val.as_bytes())?;
            }
        }

        Ok(())
    }
}
