use pixel::Pixel;
use color::Color;
use bitmap::Bitmap;

// Functions shared by all shaders
pub trait Shader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel>;
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
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel> {
        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            res.push(self.src);
        }
        res
    }
}

// Bitmap shader
pub struct BitmapShader {
    src: Bitmap,
}
impl BitmapShader {
    pub fn new(src: Bitmap) -> BitmapShader {
        BitmapShader {
            src: src,
        }
    }
}
impl Shader for BitmapShader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel> {
        vec![Pixel::pack_rgb(255, 255, 255)]
    }
}

// Shader factory
pub struct Shaders;
impl Shaders {
    pub fn from_color(color: Color) -> ColorShader {
        ColorShader::new(color)
    }
}
