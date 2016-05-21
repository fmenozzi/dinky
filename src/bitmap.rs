use pixel::Pixel;
use ppm::PPMImage;

use std::path::Path;

#[derive(Clone)]
pub struct Bitmap {
    pub width:  usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        // Transparent bitmap
        Bitmap {
            width:  width,
            height: height,
            pixels: vec![Pixel::pack_argb(0, 0, 0, 0); width*height]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, p: &Pixel) {
        self.pixels[x + y*self.width] = *p;
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        self.pixels[x + y*self.width]
    }

    pub fn read(&mut self, path: &Path) {
        let mut image = PPMImage::new(self.width, self.height);

        image.read(&path);

        self.width  = image.width;
        self.height = image.height;
        self.pixels = vec![Pixel::pack_argb(255, 255, 255, 255); self.width*self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, &image.get(x, y).to_pixel());
            }
        }
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
