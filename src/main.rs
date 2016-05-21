extern crate dinky;

use dinky::color::Color;
use dinky::pixel::Pixel;
use dinky::rect::Rect;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;
use dinky::shader::Shaders;

use std::path::Path;

fn make_circle(bitmap: &mut Bitmap, color: &Color) {
    let px = color.pin_to_unit().to_pixel();

    let (cx, cy) = (bitmap.width as f32 / 2.0, bitmap.height as f32 / 2.0);

    let r  = cx - 1.0;
    let r2 = r*r;

    for y in 0..bitmap.height {
        let dy = y as f32 - cy;
        for x in 0..bitmap.width {
            let dx = x as f32 - cx;
            let d2 = dx*dx + dy*dy;
            if d2 <= r2 {
                bitmap.set(x, y, &px);
            } else {
                bitmap.set(x, y, &Pixel::pack_argb(0,0,0,0));
            }
        }
    }
}

fn draw_solid_ramp(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(256, 196));

    let (ramp_w, ramp_h) = (1, 28);

    let c = 1.0/512.0;
    let d = 1.0/256.0;

    let recs = [
        (Color::make_argb(1.0,     c,     c,     c), Color::make_argb(0.0,   d,   d,   d)),  // Grey
        (Color::make_argb(1.0, 1.0-c,   0.0,   0.0), Color::make_argb(0.0,  -d, 0.0, 0.0)),  // Red
        (Color::make_argb(1.0,   0.0,     c,     c), Color::make_argb(0.0, 0.0,   d,   d)),  // Cyan
        (Color::make_argb(1.0,   0.0, 1.0-c,   0.0), Color::make_argb(0.0, 0.0,  -d, 0.0)),  // Green
        (Color::make_argb(1.0,     c,   0.0,     c), Color::make_argb(0.0,   d, 0.0,   d)),  // Magenta
        (Color::make_argb(1.0,   0.0,   0.0, 1.0-c), Color::make_argb(0.0, 0.0, 0.0,  -d)),  // Blue
        (Color::make_argb(1.0,     c,     c,   0.0), Color::make_argb(0.0,   d,   d, 0.0)),  // Yellow
    ];

    for y in 0..recs.len() {
        let (mut color, delta) = recs[y];
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
    let mut canvas = Canvas::new(Bitmap::new(300, 300));

    let n = 300.0;

    let mut bitmap = Bitmap::new(100, 100);
    bitmap.read(&Path::new("spock.ppm"));

    for y in 0..2 {
        for x in 0..2 {
            let (xf, yf) = (x as f32, y as f32);
            let rect = Rect::make_xywh(xf*n - n/2.0, yf*n - n/2.0, n, n);
            canvas.fill_bitmap_rect(&bitmap, &rect);
        }
    }

    canvas.write(&Path::new(&pathstr));
}

fn draw_spocks_zoom(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(300, 300));

    let n = 300.0;

    let mut bitmap = Bitmap::new(100, 100);
    bitmap.read(&Path::new("spock.ppm"));

    for i in 0..9 {
        let f = i as f32;
        let r = Rect::make_ltrb(f*10.0, f*10.0, n - f*10.0, n - f*10.0);
        canvas.fill_bitmap_rect(&bitmap, &r);
    }

    canvas.write(&Path::new(&pathstr));
}

fn draw_bm_circles(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(300, 300));

    let n = 300.0;

    let mut bitmap = Bitmap::new(n as usize, n as usize);

    let recs = [
        (Rect::make_xywh(0.0, 0.0, n, n), Color::make_argb(1.0, 1.0, 1.0, 1.0)),

        (Rect::make_xywh(  0.0,   0.0, n/2.0, n/2.0), Color::make_argb(0.8, 0.0, 0.0, 1.0)),
        (Rect::make_xywh(n/2.0,   0.0, n/2.0, n/2.0), Color::make_argb(0.6, 0.0, 1.0, 0.0)),
        (Rect::make_xywh(  0.0, n/2.0, n/2.0, n/2.0), Color::make_argb(0.4, 1.0, 0.0, 0.0)),
        (Rect::make_xywh(n/2.0, n/2.0, n/2.0, n/2.0), Color::make_argb(0.2, 0.0, 0.0, 0.0)),

        (Rect::make_xywh(  0.0, n/3.0,     n, n/3.0), Color::make_argb(0.5, 1.0, 1.0, 0.0)),
        (Rect::make_xywh(n/3.0,   0.0, n/3.0,     n), Color::make_argb(0.5, 0.0, 1.0, 1.0)),
        (Rect::make_xywh(n/3.0, n/3.0, n/3.0, n/3.0), Color::make_argb(0.5, 1.0, 0.0, 1.0)),
    ];

    for i in 0..recs.len() {
        let rect  = &recs[i].0;
        let color = &recs[i].1;

        make_circle(&mut bitmap, &color);

        canvas.fill_bitmap_rect(&bitmap, &rect);
    }

    canvas.write(&Path::new(&pathstr));
}

fn draw_circle_big(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(400, 300));

    let n = 300.0;
    let a = 0.4;

    let colors = [
        Color::make_argb(a, 1.0, 0.0, 0.0),
        Color::make_argb(a, 0.0, 1.0, 0.0),
        Color::make_argb(a, 0.0, 0.0, 1.0),
    ];

    let mut x = 0;
    let mut m = n as i32;
    let mut i = 0;
    while m > 4 {
        let mut bitmap = Bitmap::new(m as usize, m as usize);
        make_circle(&mut bitmap, &colors[i % colors.len()]);

        canvas.fill_bitmap_rect(&bitmap, &Rect::make_xywh(x as f32, 0.0, n, n));

        x += n as i32 / 12;
        m >>= 1;
        i += 1;
    }

    canvas.write(&Path::new(&pathstr));
}

fn main() {
    draw_solid_ramp("results/ppm/solid_ramp.ppm");
    draw_blend_ramp(&Color::black(), "results/ppm/blend_black.ppm");
    draw_blend_ramp(&Color::white(), "results/ppm/blend_white.ppm");

    draw_spocks_quad("results/ppm/spocks_quad.ppm");
    draw_spocks_zoom("results/ppm/spocks_zoom.ppm");
    draw_bm_circles("results/ppm/circles_blend.ppm");
    draw_circle_big("results/ppm/circles_fat.ppm");
}
