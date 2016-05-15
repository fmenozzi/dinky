use bitmap::Bitmap;
use color::Color;
use rect::Rect;
use triangle::Triangle;
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
            for _ in 0..roi.height() as usize {
                // Draw row
                for _ in 0..roi.width() as usize {
                    self.bitmap.pixels[i] = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.pixels[i])
                    };
                    i += 1;
                }

                // Advance to next row
                i += self.bitmap.width - roi.width() as usize;
            }
        }
    }

    pub fn fill_tri(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        // Vertices
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let denom = ax*by - ay*bx - ax*cy + ay*cx + bx*cy - by*cx;

        let bxcy_bycx = bx*cy - by*cx;
        let axcy_aycx = ax*cy - ay*cx;

        // TODO: Clipping
        let bounds = tri.bounds();
        for xi in bounds.xmin..bounds.xmax+1 {
            for yi in bounds.ymin..bounds.ymax+1 {
                let (x, y) = (xi as f32, yi as f32);

                // TODO: Forward difference
                let alpha_numer = bxcy_bycx + by*x - bx*y - cy*x + cx*y;
                let beta_numer  = axcy_aycx + ay*x - ax*y - cy*x + cx*y;

                let alpha =  alpha_numer / denom;
                let beta  = -beta_numer  / denom;

                if alpha >= 0f32 && beta >= 0f32 && alpha + beta <= 1f32 {
                    let i = (xi + yi * self.bitmap.width as i32) as usize;

                    self.bitmap.pixels[i] = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.pixels[i])
                    };
                }
            }
        }
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
