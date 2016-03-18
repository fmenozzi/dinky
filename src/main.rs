extern crate dinky;

use dinky::ppm::PPMImage as PPMImage;
use dinky::color::Color as Color;

use std::path::Path;

fn main() {
    let image = PPMImage::new(5, 5);

    let red = Color::new(1,0,0);
    image.set(2, 2, &red);

    let path = Path::new("out.ppm");
    image.write(&path);
}
