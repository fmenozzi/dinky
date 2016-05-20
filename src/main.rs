extern crate dinky;
extern crate cgmath;

use dinky::color::Color;
use dinky::rect::Rect;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;
use dinky::shader::Shaders;

use std::path::Path;

fn draw_solid_ramp(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(256, 196));

    let (ramp_w, ramp_h) = (1, 28);

    let c = 1.0/512.0;
    let d = 1.0/256.0;

    let rec = [
        (Color::make_argb(1.0,     c,     c,     c), Color::make_argb(0.0,   d,   d,   d)),  // Grey
        (Color::make_argb(1.0, 1.0-c,   0.0,   0.0), Color::make_argb(0.0,  -d, 0.0, 0.0)),  // Red
        (Color::make_argb(1.0,   0.0,     c,     c), Color::make_argb(0.0, 0.0,   d,   d)),  // Cyan
        (Color::make_argb(1.0,   0.0, 1.0-c,   0.0), Color::make_argb(0.0, 0.0,  -d, 0.0)),  // Green
        (Color::make_argb(1.0,     c,   0.0,     c), Color::make_argb(0.0,   d, 0.0,   d)),  // Magenta
        (Color::make_argb(1.0,   0.0,   0.0, 1.0-c), Color::make_argb(0.0, 0.0, 0.0,  -d)),  // Blue
        (Color::make_argb(1.0,     c,     c,   0.0), Color::make_argb(0.0,   d,   d, 0.0)),  // Yellow
    ];

    for y in 0..rec.len() {
        let (mut color, delta) = rec[y];
        for x in 0..256 {
            let rect = Rect::make_xywh((x*ramp_w) as f32, (y*ramp_h) as f32, ramp_w as f32, ramp_h as f32);

            canvas.shade_rect(&rect, &mut Shaders::from_color(color));

            color.a += delta.a;
            color.r += delta.r;
            color.g += delta.g;
            color.b += delta.b;
        }
    }

    canvas.write(&Path::new(&pathstr));
}

fn draw_blend_ramp(bg: &Color, pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(200, 200));
    canvas.clear(&bg);

    let mut rect = Rect::make_xywh(-25.0, -25.0, 70.0, 70.0); 

    let delta = 8.0;
    let mut i = 0;
    while i < 200 {
        let j = i as f32;

        let r = j / 200.0;
        let g = (j / 40.0).cos().abs();
        let b = (j / 50.0).sin().abs();

        let color = Color::make_argb(0.3, r, g, b);

        canvas.shade_rect(&rect, &mut Shaders::from_color(color));

        rect.offset(delta, delta);

        i += delta as i32;
    }

    canvas.write(&Path::new(&pathstr));
}

fn draw_spocks_quad(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(200, 200));

    let rect = Rect::make_xywh(70.0, 70.0, 100.0, 100.0);
    let mut bitmap = Bitmap::new(100, 100);
    bitmap.read(&Path::new("results/solid_ramp.ppm"));

    canvas.fill_bitmap_rect(bitmap, &rect);

    canvas.write(&Path::new(&pathstr));
}

fn main() {
    /*
    draw_solid_ramp("results/solid_ramp.ppm");
    draw_blend_ramp(&Color::black(), "results/blend_black.ppm");
    draw_blend_ramp(&Color::white(), "results/blend_white.ppm");
    */

    draw_spocks_quad("results/spocks_quad.ppm");
}
