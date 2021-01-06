use nalgebra::{Vector2, Vector3};
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
    pub extra_custom: Vec<f32>,
}
#[derive(Clone, Debug)]
pub struct ItemDesc {
    pub number_components: usize,
    pub size_component: usize,
    pub name: String,
}
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    //for now description only covers extra items
    pub description: Vec<ItemDesc>,
}
impl Mesh {
    /// Returns a 2x2 plane aligended with the x-y plane centerd at (0,0,0)
    pub fn plane() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    position: Vector3::new(1.0, 1.0, -0.5),
                    uv: Vector2::new(1.0, 1.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
                Vertex {
                    position: Vector3::new(1.0, -1.0, -0.5),
                    uv: Vector2::new(1.0, 0.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
                Vertex {
                    position: Vector3::new(-1.0, -1.0, -0.5),
                    uv: Vector2::new(0.0, 0.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
                //Second Triangle
                Vertex {
                    position: Vector3::new(1.0, 1.0, -0.5),
                    uv: Vector2::new(1.0, 1.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
                Vertex {
                    position: Vector3::new(-1.0, -1.0, -0.5),
                    uv: Vector2::new(0.0, 0.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
                Vertex {
                    position: Vector3::new(-1.0, 1.0, -0.5),
                    uv: Vector2::new(0.0, 1.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    extra_custom: vec![],
                },
            ],
            description: vec![],
        }
    }
}
