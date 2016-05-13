use pixel::Pixel;
use ppm::PPMImage;

use std::path::Path;

pub struct Bitmap {
    pub width:  usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        // Blank white bitmap
        Bitmap {
            width:  width,
            height: height,
            pixels: vec![Pixel::pack_argb(255, 255, 255, 255); width*height]
        }
    }

    pub fn write(&self, path: &Path) {
        let mut image = PPMImage::new(self.width, self.height);

        let mut k = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                image.set(x, y, &self.pixels[k].to_color());
                k += 1;
            }
        }

        image.write(&path);
    }
}
