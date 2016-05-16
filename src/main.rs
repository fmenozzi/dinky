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

    let white  = Color::make_argb(1.0, 1.0, 1.0, 1.0);
    let red    = Color::make_argb(1.0, 1.0, 0.0, 0.0);
    let green  = Color::make_argb(0.5,  0.0, 1.0, 0.0);
    let blue   = Color::make_argb(0.5,  0.0, 0.0, 1.0);
    let yellow = Color::make_argb(0.5, 1.0, 1.0, 0.0);

    // White canvas
    canvas.clear(&white);

    // Draw some rectangles
    canvas.fill_rect(&Rect::make_xywh(50.0,  50.0, 100.0, 50.0), &red);
    canvas.fill_rect(&Rect::make_xywh(75.0,  75.0, 50.0,  50.0), &green);
    canvas.fill_rect(&Rect::make_xywh(100.0, 30.0, 50.0, 100.0), &blue);
    canvas.fill_rect(&Rect::make_xywh(65.0,  65.0, 50.0,  50.0), &yellow);

    // Draw a convex polygon
    let (a,b,c) = (0.0, 50.0, 100.0);
    let d = 120.0;
    let center = Point2::new(b+d, b+d);
    let outside_points = [
        Point2::new(b+d, a+d),
        Point2::new(c+d, b+d),
        Point2::new(b+d, c+d),
        Point2::new(a+d, b+d),
    ];
    let colors = [
        red,
        green,
        blue,
        yellow,
    ];
    for i in 0..outside_points.len() {
        let (p0, p1) = (i, (i+1) % outside_points.len());

        let tri = Triangle {
            a: center,
            b: outside_points[p0],
            c: outside_points[p1],
        };

        canvas.fill_tri_scanline(&tri, &blue);
    }

    // Save canvas to a file
    canvas.write(&Path::new("out.ppm"));
}
