extern crate dinky;

use dinky::color::Color;
use dinky::pixel::Pixel;
use dinky::rect::Rect;
use dinky::point::Point;
use dinky::canvas::Canvas;
use dinky::bitmap::Bitmap;

use std::path::Path;

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

            canvas.fill_rect(&rect, &color);

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

        canvas.fill_rect(&rect, &color);

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

fn draw_tri(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));

    let points = [
        Point::new( 10.0,  10.0),
        Point::new(200.0,  50.0),
        Point::new(100.0, 200.0),
    ];

    canvas.fill_convex_polygon(&points, &Color::green());

    canvas.write(&Path::new(&pathstr));
}

fn draw_tri_clipped(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));

    let points = [
        Point::new(-10.0, -10.0),
        Point::new(300.0,  50.0),
        Point::new(100.0, 300.0),
    ];

    canvas.fill_convex_polygon(&points, &Color::yellow());

    canvas.write(&Path::new(&pathstr));
}

fn make_regular_poly(points: &mut[Point], count: usize, cx: f32, cy: f32, radius: f32) {
    let mut angle = 0f32;
    let delta_angle = std::f32::consts::PI*2.0 / count as f32;

    for i in 0..count {
        points[i] = Point::new(cx + angle.cos()*radius, cy + angle.sin()*radius);
        angle += delta_angle;
    }
}

fn dr_poly(canvas: &mut Canvas, dx: f32, dy: f32) {
    let mut points = [Point::new(0.0, 0.0); 12];
    for count in (3 as usize .. 13 as usize).rev() {
        make_regular_poly(&mut points, count, 256 as f32, 256 as f32, (count*10 + 120) as f32);

        for i in 0..count {
            points[i].x += dx;
            points[i].y += dy;
        }

        let color = Color::make_argb(0.8,
                                     (count as f32 *  7.0).sin().abs(),
                                     (count as f32 * 11.0).sin().abs(),
                                     (count as f32 * 17.0).sin().abs());

        canvas.fill_convex_polygon(&points[0..count], &color);
    }
}

fn draw_poly(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(512, 512));
    dr_poly(&mut canvas, 0.0, 0.0);
    canvas.write(&Path::new(&pathstr));
}

fn draw_poly_center(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(256, 256));
    dr_poly(&mut canvas, -128.0, -128.0);
    canvas.write(&Path::new(&pathstr));
}

fn scale(vec: &Point, size: f32) -> Point {
    let scale = size / (vec.x*vec.x + vec.y*vec.y).sqrt();
    Point::new(vec.x*scale, vec.y*scale)
}

fn draw_line(canvas: &mut Canvas, a: &Point, b: &Point, width: f32, color: &Color) {
    let norm = scale(&Point::new(b.y-a.y, b.x-a.x), width/2.0);

    let points = [
        Point::new(a.x - norm.x, a.y - norm.y),
        Point::new(b.x - norm.x, b.y - norm.y),
        Point::new(b.x + norm.x, b.y + norm.y),
        Point::new(a.x + norm.x, a.y + norm.y),
    ];

    canvas.fill_convex_polygon(&points, &color);
}

fn draw_poly_rotate(pathstr: &str) {
    let mut canvas = Canvas::new(Bitmap::new(230, 230));

    let start = Point::new(20.0, 20.0);
    let scale = 200f32;

    let n = 10.0;
    let mut color = Color::red();
    let delta_r = -1.0/n;
    let delta_b =  1.0/n;

    let width = 10.0;

    let mut angle = 0.0;
    while angle <= std::f32::consts::PI / 2.0 {
        let end = Point::new(start.x + angle.cos()*scale, start.y + angle.sin()*scale);

        draw_line(&mut canvas, &start, &end, width, &color);

        color.r += delta_r;
        color.b += delta_b;

        angle += std::f32::consts::PI / 2.0 / n;
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

    draw_tri("results/ppm/tri.ppm");
    draw_tri_clipped("results/ppm/tri_clipped.ppm");
    draw_poly("results/ppm/poly.ppm");
    draw_poly_center("results/ppm/poly_center.ppm");
    draw_poly_rotate("results/ppm/poly_rotate.ppm");
}
