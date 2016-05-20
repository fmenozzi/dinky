use pixel::Pixel;
use rect::Rect;

use cgmath::Matrix3;
use cgmath::prelude::SquareMatrix;

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

// TODO: Use slices instead of vectors for read access
pub fn blend_row(src: &Vec<Pixel>, dst: &Vec<Pixel>) -> Vec<Pixel> {
    assert!(src.len() == dst.len(), format!("src and dst rows not the same size ({} vs {})", src.len(), dst.len()));

    let mut res: Vec<Pixel> = Vec::with_capacity(src.len());
    for i in 0..src.len() {
        res.push(if src[i].a == 255 {src[i]} else {blend(&src[i], &dst[i])});
    }
    res
}

pub fn map_rect_to_rect_mat(src: &Rect, dst: &Rect) -> Matrix3<f32> {
    let mut res = [0f32; 6];

    let t1x = -src.left();
    let t1y = -src.top();

    let sx = dst.width() / src.width();
    let sy = dst.height() / src.height();

    let t2x = dst.left();
    let t2y = dst.top();

    res[0] = sx;
    res[1] = 0.0;
    res[2] = t1x*sx + t2x;

    res[3] = 0.0;
    res[4] = sy;
    res[5] = t1y*sy + t2y;

    let mut mat = Matrix3::new(
        res[0], res[1], res[2],
        res[3], res[4], res[5],
           0.0,    0.0,    1.0
    );
    mat.transpose_self();

    mat
}

pub fn clamp(min: f32, value: f32, max: f32) -> f32 {
    value.min(max).max(min)
}
