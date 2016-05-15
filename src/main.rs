extern crate dinky;
extern crate cgmath;

use dinky::color::Color;
use dinky::rect::Rect;
use dinky::triangle::Triangle;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;

use cgmath::Point2;

use std::path::Path;

fn main() {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));

    let white = Color::make_argb(1f32, 1f32, 1f32, 1f32);
    let red   = Color::make_argb(1f32, 1f32, 0f32, 0f32);
    let green = Color::make_argb(0.5,  0f32, 1f32, 0f32);
    let blue  = Color::make_argb(0.5,  0f32, 0f32, 1f32);

    // White canvas
    canvas.clear(&white);

    // Draw some rectangles
    canvas.fill_rect(&Rect::make_xywh(50f32,  50f32, 100f32, 50f32), &red);
    canvas.fill_rect(&Rect::make_xywh(75f32,  75f32, 50f32,  50f32), &green);
    canvas.fill_rect(&Rect::make_xywh(100f32, 30f32, 50f32, 100f32), &blue);

    // Draw a triangle
    let tri = Triangle {
        a: Point2::new(-150f32, -150f32),
        b: Point2::new(50f32, 25f32),
        c: Point2::new(25f32, 50f32),
    };
    canvas.fill_tri(&tri, &red);

    // Save canvas to a file
    canvas.write(&Path::new("out.ppm"));
}
