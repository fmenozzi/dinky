use bitmap::Bitmap;
use color::Color;
use pixel::Pixel;
use rect::Rect;
use point::Point;
use matrix::Matrix;
use triangle::Triangle;
use shader::{Shader, Shaders};
use util::{blend_row, map_rect_to_rect_mat};

use std::path::Path;
use std::cmp::{min, max};

pub struct Canvas {
    bitmap: Bitmap,
    ctms:   Vec<Matrix>,
}

impl Canvas {
    pub fn new(bitmap: Bitmap) -> Canvas {
        let ctms = vec![Matrix::identity()];
        Canvas {
            bitmap: bitmap,
            ctms:   ctms,
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

    pub fn fill_bitmap_rect(&mut self, src: &Bitmap, dst: &Rect) {
        let (w, h) = (self.bitmap.width, self.bitmap.height);
        let mut roi = Rect::make_wh(w as f32, h as f32);
        if !roi.intersect(&dst.round()) {
            return;
        }

        let srcrect = Rect::make_wh(src.width as f32, src.height as f32);

        let r2r = map_rect_to_rect_mat(&srcrect, &dst);
        let r2r_floats = [
            r2r.at(0), r2r.at(1), r2r.at(2),
            r2r.at(3), r2r.at(4), r2r.at(5),
        ];

        let mut shader = Shaders::from_bitmap_mat(src, r2r_floats);

        self.shade_rect(&roi, &mut shader);
    }

    pub fn fill_rect(&mut self, rect: &Rect, color: &Color) {
        let mut color_shader = Shaders::from_color(*color);
        self.shade_rect(&rect, &mut color_shader);
    }

    pub fn shade_rect<S: Shader>(&mut self, rect: &Rect, shader: &mut S) {
        if !rect.empty() {
            let (w, h) = (self.bitmap.width, self.bitmap.height);

            // Clip rectangle with canvas
            let mut roi = Rect::make_wh(w as f32, h as f32).round();
            if !roi.intersect(&rect.round()) {
                return;
            }

            // Split into two triangles and draw each
            let tri1 = Triangle {
                // CW
                a: Point::new(roi.left,  roi.top),
                b: Point::new(roi.right, roi.top),
                c: Point::new(roi.left, roi.bottom),
            };
            let tri2 = Triangle {
                // CW
                a: Point::new(roi.right, roi.top),
                b: Point::new(roi.right, roi.bottom),
                c: Point::new(roi.left,  roi.bottom),
            };
            self.shade_tri(&tri1, shader);
            self.shade_tri(&tri2, shader);
        }
    }

    // Courtesy of http://forum.devmaster.net/t/advanced-rasterization/6145
    // TODO: Currently requires CW vertex ordering
    fn shade_tri<S: Shader>(&mut self, tri: &Triangle, shader: &mut S) {
        let (w,h) = (self.bitmap.width, self.bitmap.height);

        // Apply CTM
        let ctm = self.get_ctm();
        let tri = Triangle {
            a: ctm.apply(&tri.a),
            b: ctm.apply(&tri.b),
            c: ctm.apply(&tri.c),
        };
        shader.set_context(ctm.get_floats());

        // Clip bounding box with canvas
        let bounds = tri.bounds();
        let mut roi = Rect::make_wh(w as f32, h as f32).round();
        if !roi.intersect(&bounds) {
            return;
        }

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
        let mut xmin_i32 = (min(x1, min(x2, x3)) + 0xf) >> 4;
        let mut xmax_i32 = (max(x1, max(x2, x3)) + 0xf) >> 4;
        let mut ymin_i32 = (min(y1, min(y2, y3)) + 0xf) >> 4;
        let mut ymax_i32 = (max(y1, max(y2, y3)) + 0xf) >> 4;
        xmin_i32 = max(xmin_i32, roi.left   as i32);
        xmax_i32 = min(xmax_i32, roi.right  as i32);
        ymin_i32 = max(ymin_i32, roi.top    as i32);
        ymax_i32 = min(ymax_i32, roi.bottom as i32);

        // Half-edge constants
        let mut c1 = dy12*x1 - dx12*y1;
        let mut c2 = dy23*x2 - dx23*y2;
        let mut c3 = dy31*x3 - dx31*y3;

        // Correct for fill convention (avoid gaps/double-draws)
        if dy12 > 0 || (dy12 == 0 && dx12 < 0) {c1 -= 1;}
        if dy23 > 0 || (dy23 == 0 && dx23 < 0) {c2 -= 1;}
        if dy31 > 0 || (dy31 == 0 && dx31 < 0) {c3 -= 1;}

        let mut cy1 = c1 + dx12*(ymin_i32 << 4) - dy12*(xmin_i32 << 4);
        let mut cy2 = c2 + dx23*(ymin_i32 << 4) - dy23*(xmin_i32 << 4);
        let mut cy3 = c3 + dx31*(ymin_i32 << 4) - dy31*(xmin_i32 << 4);

        let xmin = xmin_i32 as usize;
        let xmax = xmax_i32 as usize;
        let ymin = ymin_i32 as usize;
        let ymax = ymax_i32 as usize;

        // Rasterize
        for y in ymin..ymax {
            let mut cx1 = cy1;
            let mut cx2 = cy2;
            let mut cx3 = cy3;

            let count = xmax - xmin;
            let shaded_row = shader.shade_row(xmin, y, count);
            let mut dst_row: Vec<Pixel> = Vec::with_capacity(count);
            for i in 0..count {
                dst_row.push(self.bitmap.get(xmin + i, y));
            }
            let blended_row = blend_row(&shaded_row, &dst_row);

            for x in xmin..xmax {
                if cx1 < 0 && cx2 < 0 && cx3 < 0 {
                    self.bitmap.set(x, y, &blended_row[x-xmin]);
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

    pub fn fill_convex_polygon(&mut self, points: &[Point], color: &Color) {
        let mut color_shader = Shaders::from_color(*color);
        self.shade_convex_polygon(points, &mut color_shader);
    }

    pub fn shade_convex_polygon<S: Shader>(&mut self, points: &[Point], shader: &mut S) {
        // Reject lines and points
        if points.len() <= 2 {
            return
        }

        // Triangulate convex polygon
        let mut triangles: Vec<Triangle> = Vec::with_capacity(points.len() - 2);
        for i in 1..points.len()-1 {
            triangles.push(Triangle::new(points[0], points[i], points[i+1]));
        }

        // Shade each triangle
        for triangle in triangles {
            self.shade_tri(&triangle, shader);
        }
    }

    fn get_ctm(&mut self) -> Matrix {
        self.ctms[self.ctms.len()-1]
    }

    pub fn save(&mut self) {
        let ctm = self.get_ctm().clone();
        self.ctms.push(ctm);
    }

    pub fn restore(&mut self) {
        self.ctms.pop().unwrap();
    }

    pub fn concat(&mut self, mat: [f32; 6]) {
        let ctm = self.get_ctm().clone();
        let len = self.ctms.len();
        self.ctms[len-1] = ctm.mul(&Matrix::new(mat));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.concat([
             sx, 0.0, 0.0,
            0.0,  sy, 0.0,
        ]);
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.concat([
            1.0, 0.0, tx,
            0.0, 1.0, ty,
        ]);
    }

    pub fn rotate(&mut self, radians: f32) {
        let c = radians.cos();
        let s = radians.sin();
        self.concat([
            c, -s, 0.0,
            s,  c, 0.0,
        ]);
    }

    pub fn rotate_about(&mut self, radians: f32, point: &Point) {
        self.translate(-point.x, -point.y);
        self.rotate(radians);
        self.translate(point.x, point.y);
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
