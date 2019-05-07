
/// Pixel defines an RGBA pixel with 8-bits per channel
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    /// Creates a new pixel with given color data
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }
}

/// Defines a trait for converting some object into a Pixel Data array (Vec<u8>)
pub trait IntoPixelData {
    fn into_pixel_data(self) -> Vec<u8>;
}

impl IntoPixelData for Vec<Pixel> {
    /// Converts a Pixel array into an array of writable pixel data
    fn into_pixel_data(self) -> Vec<u8> {
        let mut out = Vec::new();
        for x in self {
            out.push(x.r);
            out.push(x.g);
            out.push(x.b);
            out.push(x.a);
        }
        out
    }
}
