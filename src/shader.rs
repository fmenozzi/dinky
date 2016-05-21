use pixel::Pixel;
use color::Color;
use point::Point;
use matrix::Matrix;
use bitmap::Bitmap;
use util::clamp;

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
    local: Matrix,
    xform: Matrix,
}
impl BitmapShader {
    pub fn new(src: Bitmap, local: Matrix) -> BitmapShader {
        BitmapShader {
            src:   src,
            local: local,
            xform: Matrix::identity(),
        }
    }
}
impl Shader for BitmapShader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel> {
        let start = Point::new(x as f32 + 0.5, y as f32 + 0.5);
        let mut lookup = self.xform.apply(&start);

        let w_1 = self.src.width  as f32 - 1.0;
        let h_1 = self.src.height as f32 - 1.0;

        let a = self.xform.at(0);
        let d = self.xform.at(3);

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
        let ctm = Matrix::new(ctm);
        self.xform = (ctm.mul(&self.local)).inv();
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
        BitmapShader::new(bitmap, Matrix::new(local))
    }
}
