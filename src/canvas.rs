use bitmap::Bitmap;
use color::Color;
use pixel::Pixel;
use rect::Rect;
use triangle::Triangle;
use util;

use cgmath::Point2;

use std::path::Path;
use std::cmp::Ordering;
use std::cmp::{min, max};

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
                a: Point2::new(roi.left,  roi.top),
                b: Point2::new(roi.left,  roi.bottom),
                c: Point2::new(roi.right, roi.bottom),
            };
            let tri2 = Triangle {
                a: Point2::new(roi.left,  roi.top),
                b: Point2::new(roi.right, roi.bottom),
                c: Point2::new(roi.right, roi.top),
            };
            self.fill_tri_halfspace(&tri1, &color);
            self.fill_tri_halfspace(&tri2, &color);
        }
    }

    fn fill_bottom_flat_tri(&mut self, tri: &Triangle, pixel: &Pixel) {
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let inv_slope_1 = (bx-ax) / (by-ay);
        let inv_slope_2 = (cx-ax) / (cy-ay);

        let (mut curr_x_1, mut curr_x_2) = (ax, ax);

        for y in ay as usize .. by as usize + 1 {
            for x in curr_x_1 as usize .. curr_x_2.floor() as usize + 1 {
                let i = (x + y*(self.bitmap.width as usize)) as usize;

                self.bitmap.pixels[i] = if pixel.a == 255 {
                   *pixel
                } else {
                    util::blend(&pixel, &self.bitmap.pixels[i])
                };

            }
            curr_x_1 += inv_slope_1;
            curr_x_2 += inv_slope_2;
        }
    }

    fn fill_top_flat_tri(&mut self, tri: &Triangle, pixel: &Pixel) {
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let inv_slope_1 = (cx-ax) / (cy-ay);
        let inv_slope_2 = (cx-bx) / (cy-by);

        let (mut curr_x_1, mut curr_x_2) = (cx, cx);

        for y in (ay as usize .. cy as usize).rev() {
            curr_x_1 -= inv_slope_1;
            curr_x_2 -= inv_slope_2;
            for x in curr_x_1 as usize .. curr_x_2.floor() as usize + 1 {
                let i = (x + y*(self.bitmap.width as usize)) as usize;

                self.bitmap.pixels[i] = if pixel.a == 255 {
                   *pixel
                } else {
                    util::blend(&pixel, &self.bitmap.pixels[i])
                };
            }
        }
    }

    pub fn fill_tri_scanline(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        // TODO: CLIPPING!

        // Sort vertices in y direction, then x direction
        let mut vertices = vec![(tri.a.x, tri.a.y), (tri.b.x, tri.b.y), (tri.c.x, tri.c.y)];
        vertices.sort_by(|a, b| {
            if a.1 < b.1 {
                Ordering::Less
            } else if a.1 > b.1 {
                Ordering::Greater
            } else {
                if a.0 < b.0 {
                    Ordering::Less
                } else if a.0 > b.0 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        });
        let (ax, ay) = (vertices[0].0, vertices[0].1);
        let (bx, by) = (vertices[1].0, vertices[1].1);
        let (cx, cy) = (vertices[2].0, vertices[2].1);

        let sorted_tri = Triangle {
            a: Point2::new(ax, ay),
            b: Point2::new(bx, by),
            c: Point2::new(cx, cy),
        };

        if (by-cy).abs() < 0.001 {
            self.fill_bottom_flat_tri(&sorted_tri, &srcpx);
        } else if (ay-by).abs() < 0.001 {
            self.fill_top_flat_tri(&sorted_tri, &srcpx);
        } else {
            let dx = 0.0; // TODO: Fill this in once other cases are working
            let dy = by;

            let d = Point2::new(dx, dy);

            let bottom_tri = Triangle {
                a: Point2::new(ax, ay),
                b: Point2::new(bx, by),
                c: d,
            };
            let top_tri = Triangle {
                a: Point2::new(bx, by),
                b: d,
                c: Point2::new(cx, cy),
            };

            self.fill_bottom_flat_tri(&bottom_tri, &srcpx);
            self.fill_top_flat_tri(&top_tri, &srcpx);
        }
    }

    pub fn fill_tri_halfspace(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        let x1 = (tri.a.x * 16.0).round() as i32;
        let x2 = (tri.b.x * 16.0).round() as i32;
        let x3 = (tri.c.x * 16.0).round() as i32;

        let y1 = (tri.a.y * 16.0).round() as i32;
        let y2 = (tri.b.y * 16.0).round() as i32;
        let y3 = (tri.c.y * 16.0).round() as i32;

        let dx12 = x1-x2;
        let dx23 = x2-x3;
        let dx31 = x3-x1;

        let dy12 = y1-y2;
        let dy23 = y2-y3;
        let dy31 = y3-y1;

        let fdx12 = dx12 << 4;
        let fdx23 = dx23 << 4;
        let fdx31 = dx31 << 4;

        let fdy12 = dy12 << 4;
        let fdy23 = dy23 << 4;
        let fdy31 = dy31 << 4;

        let xmin = (min(x1, min(x2, x3)) + 0xf) >> 4;
        let xmax = (max(x1, max(x2, x3)) + 0xf) >> 4;
        let ymin = (min(y1, min(y2, y3)) + 0xf) >> 4;
        let ymax = (max(y1, max(y2, y3)) + 0xf) >> 4;

        let mut c1 = dy12*x1 - dx12*y1;
        let mut c2 = dy23*x2 - dx23*y2;
        let mut c3 = dy31*x3 - dx31*y3;
        if dy12 < 0 || (dy12 == 0 && dx12 > 0) {c1 += 1;}
        if dy23 < 0 || (dy23 == 0 && dx23 > 0) {c2 += 1;}
        if dy31 < 0 || (dy31 == 0 && dx31 > 0) {c3 += 1;}

        let mut cy1 = c1 + dx12*(ymin << 4) - dy12*(xmin << 4);
        let mut cy2 = c2 + dx23*(ymin << 4) - dy23*(xmin << 4);
        let mut cy3 = c3 + dx31*(ymin << 4) - dy31*(xmin << 4);

        for y in ymin..ymax {
            let mut cx1 = cy1;
            let mut cx2 = cy2;
            let mut cx3 = cy3;

            for x in xmin..xmax {

                //println!("cx1, cx2, cx3: {}, {}, {}", cx1, cx2, cx3);

                if cx1 < 0 && cx2 < 0 && cx3 < 0 {
                    let i = (x + y*(self.bitmap.width as i32)) as usize;

                    self.bitmap.pixels[i] = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.pixels[i])
                    };
                }

                cx1 -= fdy12;
                cx2 -= fdy23;
                cx3 -= fdy31;
            }

            cy1 += fdx12;
            cy2 += fdx23;
            cy3 += fdx31;
        }
    }

    pub fn fill_tri(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        let (w, h) = (self.bitmap.width, self.bitmap.height);

        // Clip triangle with canvas
        let bounds = tri.bounds();
        let mut roi = Rect::make_wh(w as f32, h as f32).round();
        if !roi.intersect(&bounds) {
            return;
        }

        // Vertices
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let denom = ax*by - ay*bx - ax*cy + ay*cx + bx*cy - by*cx;

        let bxcy_bycx = bx*cy - by*cx;
        let axcy_aycx = ax*cy - ay*cx;

        let x0 = roi.left;

        for yi in roi.top as i32 .. roi.bottom.floor() as i32 {
            let y = yi as f32;

            let alpha_numer_start = bxcy_bycx + by*x0 - bx*y - cy*x0 + cx*y;
            let mut alpha_numer   = alpha_numer_start;

            let beta_numer_start = axcy_aycx + ay*x0 - ax*y - cy*x0 + cx*y;
            let mut beta_numer   = beta_numer_start;

            for xi in roi.left as i32 .. roi.right.floor() as i32 {
                alpha_numer += by - cy;
                beta_numer  += ay - cy;

                let alpha =  alpha_numer / denom;
                let beta  = -beta_numer  / denom;

                if alpha >= 0.0 && beta >= 0.0 && alpha + beta < 1.0 {
                    let i = (xi + yi*(w as i32)) as usize;

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
