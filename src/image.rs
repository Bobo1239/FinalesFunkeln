use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use vec3::Vec3;

#[derive(Debug)]
pub struct Image {
    image: Vec<Vec<Vec3>>,
}

impl Image {
    pub fn save_to_ppm(&self, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(&path)?;

        let header = format!("P3 {} {} 255\n", self.image.len(), self.image[0].len());
        file.write(header.as_bytes())?;

        for row in &self.image {
            for pixel in row {
                let pixel_val = format!("{} {} {}   ", pixel.r(), pixel.g(), pixel.b());
                file.write(pixel_val.as_bytes())?;
            }
            file.write("\n".as_bytes())?;
        }

        Ok(())
    }
}
