pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Color {
    pub fn new(r: usize, g: usize, b: usize) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
