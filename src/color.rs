pub struct Color {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn make_argb(a: f32, r: f32, g: f32, b: f32) -> Color {
        Color {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn make_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            a: 1f32,
            r: r,
            g: g,
            b: b,
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            a: self.a,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
