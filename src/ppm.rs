use color::Color;
use pixel::Pixel;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
            colors: vec![Color::white(); width*height],
        }
    }

    pub fn read(&mut self, path: &Path) {
        // Read file lines
        let f = File::open(&path).unwrap();
        let file = BufReader::new(&f);
        let mut lines: Vec<String> = Vec::new();
        for line in file.lines() {
            lines.push(line.unwrap());
        }

        // Extract image dimensions
        let dims: Vec<&str> = (&lines[1]).split(" ").collect();
        let width  = dims[0].parse::<usize>().unwrap();
        let height = dims[1].parse::<usize>().unwrap();

        // Adjust image dimensions
        self.width  = width;
        self.height = height;
        self.colors = vec![Color::white(); width*height];

        // Read remaining lines into image buffer
        let lines = &lines[2..];
        let mut i = 0;
        for line in lines {
            let components: Vec<&str> = (&line).split(" ").collect();

            let r = components[0].parse::<usize>().unwrap() as u8;
            let g = components[1].parse::<usize>().unwrap() as u8;
            let b = components[2].parse::<usize>().unwrap() as u8;

            self.colors[i] = Pixel::pack_rgb(r, g, b).to_color();

            i += 1;
        }
    }

    pub fn write(&self, path: &Path) {
        // Fill up color buffer
        let (w, h) = (self.width, self.height);
        let header = format!("P3\n");
        let dims   = format!("{} {} {}\n", w, h, 255);
        let mut bufstr = header + &dims;
        for i in 0..w*h {
            let pixel = self.colors[i].to_pixel();

            let colorstr = format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);

            bufstr = bufstr + &colorstr;
        }

        // Write color buffer to .ppm file
        let mut file = File::create(path).unwrap();
        file.write_all(bufstr.as_bytes()).unwrap();
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        self.colors[x + y*self.width] = *c;
    }
}
