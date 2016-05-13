use pixel::Pixel;

pub struct Bitmap {
    width:  usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        // Blank white bitmap
        Bitmap {
            width:  width,
            height: height,
            pixels: vec![Pixel::pack_argb(255, 255, 255, 255); width*height]
        }
    }
}
