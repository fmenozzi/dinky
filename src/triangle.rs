use point::Point;
use rect::Rect;

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    pub fn bounds(&self) -> Rect {
        let xmin = self.a.x.min(self.b.x.min(self.c.x));
        let xmax = self.a.x.max(self.b.x.max(self.c.x));
        let ymin = self.a.y.min(self.b.y.min(self.c.y));
        let ymax = self.a.y.max(self.b.y.max(self.c.y));

        Rect::make_ltrb(xmin, ymin, xmax, ymax)
    }
}
