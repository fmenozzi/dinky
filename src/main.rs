extern crate dinky;
extern crate cgmath;

use dinky::color::Color;
use dinky::rect::Rect;
use dinky::triangle::Triangle;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;

use dinky::shader::Shaders;

use cgmath::Point2;

use std::path::Path;

fn main() {
    let mut canvas = Canvas::new(Bitmap::new(500, 256));

    let white  = Color::make_argb(1.0, 1.0, 1.0, 1.0);

    let red    = Color::make_argb(1.0, 1.0, 0.0, 0.0);
    let green  = Color::make_argb(0.5, 0.0, 1.0, 0.0);
    let blue   = Color::make_argb(0.5, 0.0, 0.0, 1.0);
    let yellow = Color::make_argb(0.5, 1.0, 1.0, 0.0);

    let red_shader    = Shaders::from_color(red);
    let green_shader  = Shaders::from_color(green);
    let blue_shader   = Shaders::from_color(blue);
    let yellow_shader = Shaders::from_color(yellow);

    // White canvas
    canvas.clear(&white);

    // Draw some rectangles
    canvas.shade_rect(&Rect::make_xywh(50.0, 50.0, 100.0, 50.0), &red_shader);
    canvas.shade_rect(&Rect::make_xywh(75.0, 75.0, 50.0,  50.0), &green_shader);
    canvas.shade_rect(&Rect::make_xywh(90.0, 30.0, 50.0, 100.0), &blue_shader);
    canvas.shade_rect(&Rect::make_xywh(65.0, 65.0, 50.0,  50.0), &yellow_shader);

    // Draw a triangle
    let inside_tri = Triangle {
        a: Point2::new(50.0, 200.0),
        b: Point2::new(70.0, 230.0),
        c: Point2::new(40.0, 220.0),
    };
    canvas.shade_tri(&inside_tri, &red_shader);

    // Draw a clipped triangle
    let clipped_tri = Triangle {
        a: Point2::new( 30.0, 150.0),
        b: Point2::new( 30.0, 180.0),
        c: Point2::new(-30.0, 165.0),
    };
    canvas.shade_tri(&clipped_tri, &red_shader);

    // Draw a diamond polygon
    let (a,b,c) = (0.0, 50.0, 100.0);
    let d = 120.0;
    let center = Point2::new(b+d, b+d);
    let outside_points = [
        Point2::new(b+d, a+d),
        Point2::new(c+d, b+d),
        Point2::new(b+d, c+d),
        Point2::new(a+d, b+d),
    ];
    let color_shaders = [
        red_shader,
        green_shader,
        blue_shader,
        yellow_shader,
    ];
    for i in 0..outside_points.len() {
        let (p0, p1) = (i, (i+1) % outside_points.len());

        let tri = Triangle {
            a: center,
            b: outside_points[p0],
            c: outside_points[p1],
        };

        canvas.shade_tri(&tri, &color_shaders[i]);
    }

    // Save canvas to a file
    canvas.write(&Path::new("out.ppm"));
}
