#[cfg(not(target_arch = "wasm32"))]
mod gfx;
mod mesh;
#[cfg(target_arch = "wasm32")]
mod webgl;
#[cfg(not(target_arch = "wasm32"))]
pub use gfx::*;
pub use mesh::{ItemDesc, Mesh, Vertex};
use nalgebra::{Matrix4, Vector3};
#[cfg(target_arch = "wasm32")]
pub use webgl::*;
#[derive(Debug, Clone)]
pub struct Transform {
    scaling: Vector3<f32>,
    translation: Vector3<f32>,
}
impl Transform {
    ///Adds a delta translation
    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.translation += delta;
    }
    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.translation = translation;
    }
    pub fn scale(&mut self, delta: Vector3<f32>) {
        self.scaling += delta;
    }
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scaling = scale;
    }
    pub fn build(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.translation) * Matrix4::new_nonuniform_scaling(&self.scaling)
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            scaling: Vector3::new(1.0, 1.0, 1.0),
            translation: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
