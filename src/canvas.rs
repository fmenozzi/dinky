use bitmap::Bitmap;
use color::Color;
use pixel::Pixel;

use std::path::Path;

pub struct Canvas {
    bitmap: Bitmap, 
}

impl Canvas {
    pub fn new(bitmap: Bitmap) -> Canvas {
        Canvas {
            bitmap: bitmap,
        }
    }

    pub fn clear(&mut self, color: &Color) {
        let srcpx = color.to_pixel();

        let w = self.bitmap.width;
        let h = self.bitmap.height;

        for i in 0..w*h {
            self.bitmap.pixels[i] = srcpx;
        }
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
