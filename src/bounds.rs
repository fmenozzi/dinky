use cgmath::Point2;

pub struct BoundingBox {
    pub xmin: i32,
    pub xmax: i32,
    pub ymin: i32,
    pub ymax: i32,
}

impl BoundingBox {
    pub fn new(a: &Point2<f32>, b: &Point2<f32>, c: &Point2<f32>) -> BoundingBox {
        BoundingBox {
            xmin: a[0].min(b[0].min(c[0])) as i32,
            xmax: a[0].max(b[0].max(c[0])) as i32,
            ymin: a[1].min(b[1].min(c[1])) as i32,
            ymax: a[1].max(b[1].max(c[1])) as i32,
        }
    }
}
