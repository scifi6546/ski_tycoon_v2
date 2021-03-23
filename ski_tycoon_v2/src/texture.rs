pub use nalgebra::{Vector2, Vector4};
#[derive(Clone)]
pub struct RGBATexture {
    pub dimensions: Vector2<u32>,
    pub pixels: Vec<Vector4<u8>>,
}
impl RGBATexture {
    pub fn get_raw_vector(&self) -> Vec<u8> {
        let mut v = vec![];
        v.reserve((self.dimensions.x * self.dimensions.y * 4) as usize);
        for pixel in self.pixels.iter() {
            v.push(pixel.x);
            v.push(pixel.y);
            v.push(pixel.z);
            v.push(pixel.w);
        }
        v
    }
    pub fn constant_color(color: Vector4<u8>, dimensions: Vector2<u32>) -> Self {
        let pixels = (0..(dimensions.x * dimensions.y)).map(|_| color).collect();
        Self { dimensions, pixels }
    }
    pub fn width(&self) -> u32 {
        self.dimensions.x
    }
    pub fn height(&self) -> u32 {
        self.dimensions.y
    }
}
