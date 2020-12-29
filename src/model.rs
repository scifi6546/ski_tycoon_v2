use super::prelude::{Mesh, Texture, Transform};
use nalgebra::{Vector2, Vector3, Vector4};
#[derive(Clone)]
pub struct Model {
    pub mesh: super::graphics_engine::Mesh,
    pub texture: super::graphics_engine::RGBATexture,
    pub transform: Transform,
}
impl Model {
    pub fn from_heights(
        heights: Vec<f32>,
        dimensions: Vector2<usize>,
        transform: Transform,
    ) -> Self {
        let mut vertices = vec![];
        for x in 0..dimensions.x - 1 {
            for y in 0..dimensions.y - 1 {
                //triangle 0
                vertices.push((
                    Vector3::new(x as f32, heights[x * dimensions.y + y] as f32, y as f32),
                    Vector2::new(0.0, 0.0),
                ));
                vertices.push((
                    Vector3::new(
                        x as f32,
                        heights[x * dimensions.y + y + 1] as f32,
                        y as f32 + 1.0,
                    ),
                    Vector2::new(0.0, 1.0),
                ));
                vertices.push((
                    Vector3::new(
                        x as f32 + 1.0,
                        heights[(x + 1) * dimensions.y + y] as f32,
                        y as f32,
                    ),
                    Vector2::new(1.0, 0.0),
                ));
                //triangle 1
                vertices.push((
                    Vector3::new(
                        x as f32,
                        heights[x * dimensions.y + y + 1] as f32,
                        y as f32 + 1.0,
                    ),
                    Vector2::new(0.0, 1.0),
                ));
                vertices.push((
                    Vector3::new(
                        x as f32 + 1.0,
                        heights[(x + 1) * dimensions.y + y + 1] as f32,
                        y as f32 + 1.0,
                    ),
                    Vector2::new(1.0, 1.0),
                ));
                vertices.push((
                    Vector3::new(
                        x as f32 + 1.0,
                        heights[(x + 1) * dimensions.y + y] as f32,
                        y as f32,
                    ),
                    Vector2::new(1.0, 0.0),
                ));
            }
        }
        Model {
            mesh: Mesh { vertices },
            texture: Texture::constant_color(Vector4::new(200, 200, 200, 255), Vector2::new(8, 8)),
            transform,
        }
    }
    pub fn cube(transform: Transform) -> Model {
        let vertices = vec![
            (Vector3::new(-1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(1.0, 1.0, 1.0), Vector2::new(1.0, 1.0)),
            (Vector3::new(1.0, -1.0, 1.0), Vector2::new(1.0, 0.0)),
            //second triangle
            (Vector3::new(-1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, 1.0, 1.0), Vector2::new(0.0, 1.0)),
            (Vector3::new(1.0, 1.0, 1.0), Vector2::new(1.0, 1.0)),
            //third triangle
            (Vector3::new(1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(1.0, 1.0, -1.0), Vector2::new(0.0, 1.0)),
            (Vector3::new(1.0, -1.0, -1.0), Vector2::new(0.0, 1.0)),
            //fourth triangle
            (Vector3::new(1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(1.0, 1.0, 1.0), Vector2::new(0.0, 1.0)),
            (Vector3::new(1.0, 1.0, -1.0), Vector2::new(1.0, 1.0)),
            //fith triangle
            (Vector3::new(1.0, -1.0, -1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(1.0, 0.0)),
            (Vector3::new(1.0, 1.0, -1.0), Vector2::new(0.0, 1.0)),
            //sixth triangle
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(1.0, 0.0)),
            (Vector3::new(-1.0, 1.0, -1.0), Vector2::new(1.0, 1.0)),
            (Vector3::new(1.0, 1.0, -1.0), Vector2::new(0.0, 1.0)),
            //seventh triangle
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, -1.0, 1.0), Vector2::new(1.0, 0.0)),
            (Vector3::new(-1.0, 1.0, 1.0), Vector2::new(1.0, 1.0)),
            //eighth triangle
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, 1.0, 1.0), Vector2::new(1.0, 1.0)),
            (Vector3::new(-1.0, 1.0, -1.0), Vector2::new(0.0, 1.0)),
            //9th triangle
            (Vector3::new(1.0, 1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(1.0, 1.0, -1.0), Vector2::new(0.0, 1.0)),
            (Vector3::new(-1.0, 1.0, -1.0), Vector2::new(1.0, 1.0)),
            //10th triangle
            (Vector3::new(1.0, 1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, 1.0, -1.0), Vector2::new(1.0, 1.0)),
            (Vector3::new(-1.0, 1.0, 1.0), Vector2::new(0.0, 1.0)),
            //11th triangle
            (Vector3::new(1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, -1.0, 1.0), Vector2::new(1.0, 0.0)),
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(1.0, 1.0)),
            //12th triangle
            (Vector3::new(1.0, -1.0, 1.0), Vector2::new(0.0, 0.0)),
            (Vector3::new(-1.0, -1.0, -1.0), Vector2::new(1.0, 1.0)),
            (Vector3::new(1.0, -1.0, -1.0), Vector2::new(0.0, 1.0)),
        ];
        Model {
            mesh: Mesh { vertices },
            texture: Texture::constant_color(Vector4::new(255, 0, 0, 255), Vector2::new(8, 8)),
            transform,
        }
    }
}
