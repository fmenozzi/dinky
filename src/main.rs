extern crate dinky;

use dinky::ppm::PPMImage;
use dinky::color::Color;

use std::path::Path;

fn main() {
    let mut image = PPMImage::new(256, 256);

    image.set(2, 2, &Color::make_rgb(1f32, 0f32, 0f32));
    image.write(&Path::new("out.ppm"));
}
