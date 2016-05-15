use pixel::Pixel;

pub fn blend(src: &Pixel, dst: &Pixel) -> Pixel {
    let magic: u32 = (1<<16) | (1<<8) | 1;

    let (dst_a, src_a) = (dst.a as u32, src.a as u32);
    let (dst_r, src_r) = (dst.r as u32, src.r as u32);
    let (dst_g, src_g) = (dst.g as u32, src.g as u32);
    let (dst_b, src_b) = (dst.b as u32, src.b as u32);

    let sub: u32 = 255 - src_a;

    let final_a = src_a + (((sub * dst_a * magic) + (1<<23)) >> 24);
    let final_r = src_r + (((sub * dst_r * magic) + (1<<23)) >> 24);
    let final_g = src_g + (((sub * dst_g * magic) + (1<<23)) >> 24);
    let final_b = src_b + (((sub * dst_b * magic) + (1<<23)) >> 24);

    Pixel::pack_argb(final_a as u8, final_r as u8, final_g as u8, final_b as u8)
}
