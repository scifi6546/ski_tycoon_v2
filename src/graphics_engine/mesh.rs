use nalgebra::{Vector2, Vector3};
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<(Vector3<f32>, Vector2<f32>)>,
}
impl Mesh {
    /// Returns a 2x2 plane aligended with the x-y plane centerd at (0,0,0)
    pub fn plane() -> Self {
        Self {
            vertices: vec![
                (Vector3::new(1.0, 1.0, 0.0), Vector2::new(1.0, 1.0)),
                (Vector3::new(-1.0, -1.0, 0.0), Vector2::new(0.0, 0.0)),
                (Vector3::new(1.0, -1.0, 0.0), Vector2::new(1.0, 0.0)),
                //Second Triangle
                (Vector3::new(1.0, 1.0, 0.0), Vector2::new(1.0, 1.0)),
                (Vector3::new(-1.0, -1.0, 0.0), Vector2::new(0.0, 0.0)),
                (Vector3::new(-1.0, -1.0, 0.0), Vector2::new(0.0, 1.0)),
            ],
        }
    }
}
