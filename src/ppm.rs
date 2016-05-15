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
            colors: vec![Color{a: 1.0, r: 1.0, g: 1.0, b: 1.0}; width*height],
        }
    }

    pub fn write(&self, path: &Path) {
        // Fill up color buffer
        let header = format!("P3\n");
        let dims   = format!("{} {} {}\n", self.width, self.height, 255);
        let mut bufstr = header + &dims;
        for i in 0..(self.width*self.height) {
            let r = (self.colors[i].r * 255.0) as i32;
            let g = (self.colors[i].g * 255.0) as i32;
            let b = (self.colors[i].b * 255.0) as i32;

            let c = format!("{} {} {}\n", r, g, b);

            bufstr = bufstr + &c;
        }

        // Write color buffer to .ppm file
        let mut file = File::create(path).unwrap();
        file.write_all(bufstr.as_bytes()).unwrap();
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        self.colors[y + x*self.width].a = c.a;
        self.colors[y + x*self.width].r = c.r;
        self.colors[y + x*self.width].g = c.g;
        self.colors[y + x*self.width].b = c.b;
    }
}
