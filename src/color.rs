use pixel::Pixel;

#[derive(Copy, Clone)]
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

    pub fn pin_to_unit(&self) -> Color {
        // TODO: Change these to a clamp() call
        let a = 0f32.max(self.a.min(1f32));
        let r = 0f32.max(self.r.min(1f32));
        let g = 0f32.max(self.g.min(1f32));
        let b = 0f32.max(self.b.min(1f32));

        Color::make_argb(a,r,g,b)
    }

    pub fn to_pixel(&self) -> Pixel {
        let uc = self.pin_to_unit();

        let a255 = uc.a * 255.9999f32;

        let a = a255 as u8;
        let r = (uc.r * a255) as u8;
        let g = (uc.g * a255) as u8;
        let b = (uc.b * a255) as u8;

        Pixel::pack_argb(a,r,g,b)
    }
}
