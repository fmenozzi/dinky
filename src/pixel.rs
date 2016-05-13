pub struct Pixel {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn pack_argb(a: u8, r: u8, g: u8, b: u8) -> Pixel {
        // Assert a,r,g,b are already in premul form
        assert!(a <= 255, "a > 255");
        assert!(r <= a,   "r > a");
        assert!(g <= a,   "g > a");
        assert!(b <= a,   "b > a");

        Pixel {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }
}
