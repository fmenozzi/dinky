use bitmap::Bitmap;
use color::Color;
use pixel::Pixel;
use rect::Rect;
use triangle::Triangle;
use util;

use cgmath::Point2;

use std::path::Path;
use std::cmp::Ordering;

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
                b: Point2::new(roi.right, roi.bottom),
                c: Point2::new(roi.left,  roi.bottom),
            };
            let tri2 = Triangle {
                a: Point2::new(roi.left,  roi.top),
                b: Point2::new(roi.right, roi.top),
                c: Point2::new(roi.right, roi.bottom),
            };
            self.fill_tri_exp(&tri1, &color);
            self.fill_tri_exp(&tri2, &color);
        }
    }

    fn fill_bottom_flat_tri(&mut self, tri: &Triangle, pixel: &Pixel) {
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let inv_slope_1 = (bx-ax) / (by-ay);
        let inv_slope_2 = (cx-ax) / (cy-ay);

        let (mut curr_x_1, mut curr_x_2) = (ax, ax);

        for y in ay as usize .. by.floor() as usize + 1 {
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

        for y in (ay as usize .. cy.floor() as usize).rev() {
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

    pub fn fill_tri_exp(&mut self, tri: &Triangle, color: &Color) {
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
            //println!("BOTTOM");
            self.fill_bottom_flat_tri(&sorted_tri, &srcpx);
        } else if (ay-by).abs() < 0.001 {
            //println!("TOP");
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
