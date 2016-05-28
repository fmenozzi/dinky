use color::Color;
use pixel::Pixel;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub struct PPMImage {
    pub width:  usize,
    pub height: usize,
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
        // Read file nums
        let mut file = File::open(path).unwrap();
        let mut as_string = String::new();
        file.read_to_string(&mut as_string).unwrap();
        let nums: Vec<usize> = as_string.split_whitespace()
                                        .filter_map(|s| s.trim().parse::<usize>().ok())
                                        .collect();

        // Extract image dimensions
        let (width, height) = (nums[0], nums[1]);

        // Adjust image dimensions
        self.width  = width;
        self.height = height;
        self.colors = vec![Color::white(); width*height];

        // Read remaining nums into image buffer
        let nums = &nums[3..];
        for i in 0..width*height {
            let r = nums[3*i] as u8;
            let g = nums[3*i + 1] as u8;
            let b = nums[3*i + 2] as u8;

            self.colors[i] = Pixel::pack_rgb(r, g, b).to_color();
        }
    }

    pub fn write(&self, path: &Path) {
        let (w, h) = (self.width, self.height);

        let mut file = BufWriter::with_capacity(w*h*20, File::create(&path).unwrap());

        write!(file, "P3\n").unwrap();
        write!(file, "{} {} {}\n", w, h, 255).unwrap();

        for i in 0..w*h {
            let pixel = self.colors[i].to_pixel();

            // Undo premul
            let (a, mut r, mut g, mut b) = (pixel.a, pixel.r, pixel.g, pixel.b);
            if a != 0 && a != 255 {
                r = ((r as i32 * 255 + a as i32/2) / a as i32) as u8;
                g = ((g as i32 * 255 + a as i32/2) / a as i32) as u8;
                b = ((b as i32 * 255 + a as i32/2) / a as i32) as u8;
            }

            write!(file, "{} {} {}\n", r, g, b).unwrap();
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        self.colors[x + y*self.width] = *c;
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.colors[x + y*self.width]
    }
}
