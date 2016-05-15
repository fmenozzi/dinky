use cgmath::Point2;
use rect::Rect;

pub struct Triangle {
    pub a: Point2<f32>,
    pub b: Point2<f32>,
    pub c: Point2<f32>,
}


impl Triangle {
    pub fn new(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    pub fn bounds(&self) -> Rect {
        let xmin = self.a[0].min(self.b[0].min(self.c[0]));
        let xmax = self.a[0].max(self.b[0].max(self.c[0]));
        let ymin = self.a[1].min(self.b[1].min(self.c[1]));
        let ymax = self.a[1].max(self.b[1].max(self.c[1]));

        let mut rect = Rect{left: 0.0, top: 0.0, right: 0.0, bottom: 0.0};
        rect.set_ltrb(xmin, ymin, xmax, ymax);
        rect
    }
}
