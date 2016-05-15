use cgmath::Point2;
use bounds::BoundingBox;

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

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::new(&self.a, &self.b, &self.c)
    }
}
