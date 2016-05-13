use color::Color as Color;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct PPMImage {
    width:  usize,
    height: usize,
    colors: Vec<Color>,
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> PPMImage {
        // Blank white image
        PPMImage {
            width:  width,
            height: height,
            colors: vec![Color{a: 1f32, r: 1f32, g: 1f32, b: 1f32}; width*height],
        }
    }

    pub fn write(&self, path: &Path) {
        // Fill up color buffer
        let header = format!("P3\n");
        let dims   = format!("{} {} {}\n", self.width, self.height, 255);
        let mut bufstr = header + &dims;
        for i in 0..(self.width*self.height) {
            let r = (self.colors[i].r * 255f32) as i32;
            let g = (self.colors[i].g * 255f32) as i32;
            let b = (self.colors[i].b * 255f32) as i32;

            let c = format!("{} {} {}\n", r, g, b);

            bufstr = bufstr + &c;
        }

        // Write color buffer to .ppm file
        let mut file = File::create(path).unwrap();
        file.write_all(bufstr.as_bytes()).unwrap();
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        self.colors[x + y*self.width].a = c.a;
        self.colors[x + y*self.width].r = c.r;
        self.colors[x + y*self.width].g = c.g;
        self.colors[x + y*self.width].b = c.b;
    }
}
