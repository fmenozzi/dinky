use pixel::Pixel;
use color::Color;

pub trait Shader {
    fn shade_row(&self, x: usize, y: usize, count: usize) -> Vec<Pixel>;
}

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

pub struct Shaders;

impl Shaders {
    pub fn from_color(color: Color) -> ColorShader {
        ColorShader::new(color)
    }
}

