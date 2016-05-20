use pixel::Pixel;
use color::Color;
use rect::Rect;
use bitmap::Bitmap;
use util::clamp;

use cgmath::Point3;
use cgmath::Matrix3;
use cgmath::prelude::{SquareMatrix, EuclideanSpace};

// Functions shared by all shaders
pub trait Shader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel>;
    fn set_context(&mut self, ctm: [f32; 6]) -> bool;
}

// Color shader
pub struct ColorShader {
    src: Pixel,
}
impl ColorShader {
    pub fn new(src: Color) -> ColorShader {
        ColorShader {
            src: src.to_pixel(),
        } 
    }
}
impl Shader for ColorShader {
    #[allow(unused_variables)]
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel> {
        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            res.push(self.src);
        }
        res
    }

    #[allow(unused_variables)]
    fn set_context(&mut self, ctm: [f32; 6]) -> bool {
        true
    }
}

// Bitmap shader
pub struct BitmapShader {
    src:   Bitmap,
    local: Matrix3<f32>,
    xform: Matrix3<f32>,
}
impl BitmapShader {
    pub fn new(src: Bitmap, local: Matrix3<f32>) -> BitmapShader {
        BitmapShader {
            src: src,
            local: local,
            xform: Matrix3::identity(),
        }
    }
}
impl Shader for BitmapShader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel> {
        let start = Point3::new(x as f32 + 0.5, y as f32 + 0.5, 1.0);
        let mut lookup = self.xform * start.to_vec();

        let w_1 = self.src.width  as f32 - 1.0;
        let h_1 = self.src.height as f32 - 1.0;

        let a = self.xform[0][0];
        let d = self.xform[1][0];

        let mut res = Vec::with_capacity(count);

        for _ in 0..count {
            let lookup_x = clamp(0.0, lookup.x, w_1) as usize;
            let lookup_y = clamp(0.0, lookup.y, h_1) as usize;

            res.push(self.src.get(lookup_x, lookup_y));

            lookup.x += a;
            lookup.y += d;
        }

        res
    }

    fn set_context(&mut self, ctm: [f32; 6]) -> bool {
        let mut ctm = Matrix3::new(
            ctm[0], ctm[1], ctm[2],
            ctm[3], ctm[4], ctm[5],
               0.0,    0.0,    1.0
        );
        ctm.transpose_self(); // cgmath uses column-major order
        self.xform = (ctm * self.local).invert().unwrap();
        true
    }
}

// Shader factory
pub struct Shaders;
impl Shaders {
    pub fn from_color(color: Color) -> ColorShader {
        ColorShader::new(color)
    }

    pub fn from_bitmap_mat(bitmap: Bitmap, local: [f32; 6]) -> BitmapShader {
        let mut mat = Matrix3::new(
            local[0], local[1], local[2],
            local[3], local[4], local[5],
                 0.0,      0.0,      1.0
        );
        mat.transpose_self();
        BitmapShader::new(bitmap, mat)
    }
    pub fn from_bitmap_rect(bitmap: Bitmap, dst: Rect) -> BitmapShader {
        let (bw, bh) = (bitmap.width as f32, bitmap.height as f32);
        let (rw, rh) = (dst.width(), dst.height());

        let mat = [
            rw/bw,  0.0,    dst.left(),
            0.0,    rh/bh,  dst.top(),
        ];

        Shaders::from_bitmap_mat(bitmap, mat)
    }
}
