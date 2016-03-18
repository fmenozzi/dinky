use color::Color as Color;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct PPMImage {
        width:  usize,
        height: usize,
    pub colors: Vec<Color>,
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> PPMImage {
        PPMImage {
            width:  width,
            height: height,
            colors: vec![Color{r: 0, g: 0, b: 0}; width*height],
        }
    }

    pub fn write(&self, path: &Path) {
        // Fill up color buffer
        let header = format!("P3\n");
        let dims   = format!("{} {} {}\n", self.width, self.height, 255);
        let mut bufstr = header + &dims;
        for i in 0..(self.width*self.height) {
            let r = self.colors[i].r;
            let g = self.colors[i].g;
            let b = self.colors[i].b;

            let c = format!("{} {} {}\n", r, g, b);

            bufstr = bufstr + &c;
        }

        // Write color buffer to .ppm file
        let mut file = File::create(path).unwrap();
        file.write_all(bufstr.as_bytes()).unwrap();
    }

    pub fn set(&self, x: usize, y: usize, c: &Color) {
        self.colors[x + y*self.width] = c;
    }
}
