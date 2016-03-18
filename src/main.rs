extern crate dinky;

use dinky::ppm::PPMImage as PPMImage;
use dinky::color::Color as Color;

use std::path::Path;

fn main() {
    let mut image = PPMImage::new(256, 256);

    let red = Color::new(255,0,0);
    image.set(2, 2, &red);

    let path = Path::new("out.ppm");
    image.write(&path);
}
