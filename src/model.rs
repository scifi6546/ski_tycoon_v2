use super::prelude::{Mesh, Texture, Transform, Vertex};
use nalgebra::{Vector2, Vector3, Vector4};
use std::collections::HashMap;
#[derive(Clone)]
pub struct Model {
    pub mesh: super::graphics_engine::Mesh,
    pub texture: Texture,
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
                let x0_y0 = Vector3::new(x as f32, heights[x * dimensions.y + y] as f32, y as f32);
                let x0_y1 = Vector3::new(
                    x as f32,
                    heights[x * dimensions.y + y + 1] as f32,
                    y as f32 + 1.0,
                );
                let x1_y0 = Vector3::new(
                    x as f32 + 1.0,
                    heights[(x + 1) * dimensions.y + y] as f32,
                    y as f32,
                );
                let x1_y1 = Vector3::new(
                    x as f32 + 1.0,
                    heights[(x + 1) * dimensions.y + y + 1] as f32,
                    y as f32 + 1.0,
                );
                let triangle0_normal = (x0_y1 - x0_y0).cross(&(x1_y0 - x0_y0)).normalize();
                let triangle1_normal = (x1_y0 - x1_y1).cross(&(x0_y1 - x1_y1)).normalize();
                //triangle 0
                vertices.push(Vertex {
                    position: x0_y0,
                    uv: Vector2::new(0.0, 0.0),
                    normal: triangle0_normal,
                });
                vertices.push(Vertex {
                    position: x0_y1,
                    uv: Vector2::new(0.0, 1.0),
                    normal: triangle0_normal,
                });
                vertices.push(Vertex {
                    position: x1_y0,
                    uv: Vector2::new(1.0, 0.0),
                    normal: triangle0_normal,
                });
                //triangle 1
                vertices.push(Vertex {
                    position: x0_y1,
                    uv: Vector2::new(0.0, 1.0),
                    normal: triangle1_normal,
                });
                vertices.push(Vertex {
                    position: x1_y1,
                    uv: Vector2::new(1.0, 1.0),
                    normal: triangle1_normal,
                });
                vertices.push(Vertex {
                    position: x1_y0,
                    uv: Vector2::new(1.0, 0.0),
                    normal: triangle1_normal,
                });
            }
        }
        Model {
            mesh: Mesh {
                vertices,
                custom_attributes: HashMap::new(),
            },
            texture: Texture::constant_color(Vector4::new(200, 200, 200, 255), Vector2::new(8, 8)),
            transform,
        }
    }
    pub fn cube(transform: Transform) -> Model {
        let vertices = vec![
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //second triangle
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //third triangle
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //fourth triangle
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //fith triangle
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            //sixth triangle
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            //seventh triangle
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //eighth triangle
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            //9th triangle
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            //10th triangle
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            //11th triangle
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            //12th triangle
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
        ];
        Model {
            mesh: Mesh {
                vertices,
                custom_attributes: HashMap::new(),
            },
            texture: Texture::constant_color(Vector4::new(255, 0, 0, 255), Vector2::new(8, 8)),
            transform,
        }
    }
}
