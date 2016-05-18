use bitmap::Bitmap;
use color::Color;
use rect::Rect;
use triangle::Triangle;
use util;

use cgmath::Point2;

use std::path::Path;
use std::cmp::{Ordering, min, max};

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

            // Split into two triangles and draw each
            let tri1 = Triangle {
                // CW
                a: Point2::new(roi.left,  roi.top),
                b: Point2::new(roi.right, roi.bottom),
                c: Point2::new(roi.left,  roi.bottom),
            };
            let tri2 = Triangle {
                // CW
                a: Point2::new(roi.left,  roi.top),
                b: Point2::new(roi.right, roi.top),
                c: Point2::new(roi.right, roi.bottom),
            };
            self.fill_tri(&tri1, &color);
            self.fill_tri(&tri2, &color);
        }
    }

    // Courtesy of http://forum.devmaster.net/t/advanced-rasterization/6145
    // TODO: Currently requires CW vertex ordering
    pub fn fill_tri(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        let (w,h) = (self.bitmap.width, self.bitmap.height);

        // Clip triangle with canvas
        let bounds = tri.bounds();
        let mut roi = Rect::make_wh(w as f32, h as f32).round();
        if !roi.intersect(&bounds) {
            return;
        }

        // Get lone x coordinate
        let lone_x = if tri.a.y == tri.b.y {
            tri.c.x
        } else if tri.b.y == tri.c.y {
            tri.a.x
        } else if tri.c.y == tri.a.y {
            tri.b.x
        } else {
            -1.0
        };

        // 28.4 fixed-point coordinates
        let x1 = (tri.a.x * 16.0).round() as i32;
        let x2 = (tri.b.x * 16.0).round() as i32;
        let x3 = (tri.c.x * 16.0).round() as i32;

        let y1 = (tri.a.y * 16.0).round() as i32;
        let y2 = (tri.b.y * 16.0).round() as i32;
        let y3 = (tri.c.y * 16.0).round() as i32;

        // Deltas
        let dx12 = x1-x2;
        let dx23 = x2-x3;
        let dx31 = x3-x1;

        let dy12 = y1-y2;
        let dy23 = y2-y3;
        let dy31 = y3-y1;

        // Fixed-point deltas
        let fdx12 = dx12 << 4;
        let fdx23 = dx23 << 4;
        let fdx31 = dx31 << 4;

        let fdy12 = dy12 << 4;
        let fdy23 = dy23 << 4;
        let fdy31 = dy31 << 4;

        // Apply clipping
        let mut xmin = (min(x1, min(x2, x3)) + 0xf) >> 4;
        let mut xmax = (max(x1, max(x2, x3)) + 0xf) >> 4;
        let mut ymin = (min(y1, min(y2, y3)) + 0xf) >> 4;
        let mut ymax = (max(y1, max(y2, y3)) + 0xf) >> 4;
        xmin = max(xmin, roi.left   as i32);
        xmax = min(xmax, roi.right  as i32);
        ymin = max(ymin, roi.top    as i32);
        ymax = min(ymax, roi.bottom as i32);

        // Half-edge constants
        let mut c1 = dy12*x1 - dx12*y1;
        let mut c2 = dy23*x2 - dx23*y2;
        let mut c3 = dy31*x3 - dx31*y3;

        // Correct for fill convention (avoid gaps/double-draws)
        if dy12 > 0 || (dy12 == 0 && dx12 < 0) {c1 -= 1;}
        if dy23 > 0 || (dy23 == 0 && dx23 < 0) {c2 -= 1;}
        if dy31 > 0 || (dy31 == 0 && dx31 < 0) {c3 -= 1;}

        let mut cy1 = c1 + dx12*(ymin << 4) - dy12*(xmin << 4);
        let mut cy2 = c2 + dx23*(ymin << 4) - dy23*(xmin << 4);
        let mut cy3 = c3 + dx31*(ymin << 4) - dy31*(xmin << 4);

        // Rasterize
        for y in ymin as usize .. ymax as usize {
            let mut cx1 = cy1;
            let mut cx2 = cy2;
            let mut cx3 = cy3;

            // Determine row start and end
            let mut x0 = lone_x as usize;
            let mut x1 = if y == ymin as usize {lone_x as usize} else {xmax as usize};
            let mut started = false;
            for x in xmin as usize .. xmax as usize {
                if cx1 < 0 && cx2 < 0 && cx3 < 0 {
                    if !started {
                        x0 = x;
                        started = true;
                    }
                } else {
                    if started {
                        x1 = x;
                        break;
                    }
                }

                cx1 -= fdy12;
                cx2 -= fdy23;
                cx3 -= fdy31;
            }
            for x in x0..x1 {
                let px = if src_a == 255 {
                    srcpx
                } else {
                    util::blend(&srcpx, &self.bitmap.get(x,y))
                };

                self.bitmap.set(x, y, &px);
            }

            /*
            for x in xmin as usize .. xmax as usize {
                if cx1 < 0 && cx2 < 0 && cx3 < 0 {
                    let px = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.get(x,y))
                    };

                    self.bitmap.set(x, y, &px);
                }

                cx1 -= fdy12;
                cx2 -= fdy23;
                cx3 -= fdy31;
            }
            */

            cy1 += fdx12;
            cy2 += fdx23;
            cy3 += fdx31;
        }
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
