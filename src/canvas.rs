use bitmap::Bitmap;
use color::Color;
use rect::Rect;
use util;

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

    pub fn fill_rect(&mut self, rect: &Rect, color: &Color) {
        if !rect.empty() {
            let srcpx = color.to_pixel();

            // Skip transparent fill colors
            let src_a = srcpx.a;
            if src_a == 0 {
                return;
            }

            let (w, h) = (self.bitmap.width, self.bitmap.height);

            // Clip rectangle with canvas
            let mut roi = Rect::make_wh(w as f32, h as f32).round();
            if !roi.intersect(&rect.round()) {
                return;
            }

            let mut i = (roi.top * w as f32 + roi.left) as usize;
            for _ in 0..rect.height() as usize {
                // Draw row
                for _ in 0..rect.width() as usize {
                    self.bitmap.pixels[i] = if src_a == 255 {srcpx} else {util::blend(&srcpx, &self.bitmap.pixels[i])};
                    i += 1;
                }

                // Advance to next row
                i += self.bitmap.width - rect.width() as usize;
            }
        }
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
