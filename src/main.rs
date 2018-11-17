extern crate finales_funkeln;

use finales_funkeln::image::Image;
use finales_funkeln::vec3::Vec3;

fn main() {
    let width = 800;
    let height = 600;
    let mut image = Image::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.2;
            image.set_pixel(x, y, Vec3::new(r, g, b));
        }
    }
}
