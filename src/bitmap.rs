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

    pub fn set(&mut self, x: usize, y: usize, p: &Pixel) {
        let i = x + y*self.width;

        self.pixels[i].a = p.a;
        self.pixels[i].r = p.r;
        self.pixels[i].g = p.g;
        self.pixels[i].b = p.b;
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        let i = x + y*self.width;
        self.pixels[i]
    }

    pub fn write(&self, path: &Path) {
        let mut image = PPMImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                image.set(x, y, &self.get(x, y).to_color());
            }
        }

        image.write(&path);
    }
}
