pub struct Color {
    pub a: usize,
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Color {
    pub fn make_argb(a: usize, r: usize, g: usize, b: usize) -> Color {
        Color {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn make_rgb(r: usize, g: usize, b: usize) -> Color {
        Color {
            a: 1,
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
