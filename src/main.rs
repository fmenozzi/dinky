extern crate dinky;

use dinky::color::Color;
use dinky::rect::Rect;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;

use std::path::Path;

fn main() {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));

    let white = Color::make_argb(1f32, 1f32, 1f32, 1f32);
    let red   = Color::make_argb(1f32, 1f32, 0f32, 0f32);
    let green = Color::make_argb(0.5,  0f32, 1f32, 0f32);
    let blue  = Color::make_argb(0.5,  0f32, 0f32, 1f32);

    canvas.clear(&white);

    canvas.fill_rect(&Rect::make_xywh(50f32,  50f32, 100f32, 50f32), &red);
    canvas.fill_rect(&Rect::make_xywh(75f32,  75f32, 50f32,  50f32), &green);
    canvas.fill_rect(&Rect::make_xywh(100f32, 30f32, 50f32, 100f32), &blue);

    canvas.write(&Path::new("out.ppm"));
}
