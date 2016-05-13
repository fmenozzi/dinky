extern crate dinky;

use dinky::color::Color;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;

use std::path::Path;

fn main() {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));

    canvas.clear(&Color::make_rgb(1f32, 0f32, 0f32));
    canvas.write(&Path::new("out.ppm"));
}
