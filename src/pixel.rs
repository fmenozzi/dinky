use color::Color;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn pack_argb(a: u8, r: u8, g: u8, b: u8) -> Pixel {
        // Assert r,g,b are already in premul form
        assert!(r <= a, "r > a");
        assert!(g <= a, "g > a");
        assert!(b <= a, "b > a");

        Pixel {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn pack_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel::pack_argb(255, r, g, b)
    }

    pub fn to_color(&self) -> Color {
        let a = (self.a as f32) / 256.0;
        let r = (self.r as f32) / 256.0;
        let g = (self.g as f32) / 256.0;
        let b = (self.b as f32) / 256.0;

        Color::make_argb(a,r,g,b)
    }
}
