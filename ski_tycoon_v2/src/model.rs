use super::prelude::{ItemDesc, Mesh, Texture, Transform, Vertex};
use nalgebra::{Vector2, Vector3, Vector4};
#[derive(Clone)]
pub struct Model {
    pub mesh: super::graphics_engine::Mesh,
    pub texture: Texture,
    pub transform: Transform,
}
impl Model {
    pub fn from_obj(
        obj: &'static [u8],
        mtl: &'static [u8],
        transform: Transform,
        texture: Texture,
    ) -> Self {
        let mesh = Mesh::from_obj(obj, mtl);
        Self {
            mesh,
            texture,
            transform,
        }
    }
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
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x0_y0.x,x0_y0.y,x0_y0.z,
                        //uv
                        0.0,0.0,
                        //normal
                        triangle0_normal.x,triangle0_normal.y,triangle0_normal.z

                    ],
                });
                vertices.push(Vertex {
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x0_y1.x,x0_y1.y,x0_y1.z,
                        //uv
                        0.0,1.0,
                        //normal
                        triangle0_normal.x,triangle0_normal.y,triangle0_normal.z

                    ],
                });
                vertices.push(Vertex {
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x1_y0.x,x1_y0.y,x1_y0.z,
                        //uv
                        1.0,0.0,
                        //normal
                        triangle0_normal.x,triangle0_normal.y,triangle0_normal.z
                    ],
                });
                //triangle 1
                vertices.push(Vertex {
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x0_y1.x,x0_y1.y,x0_y1.z,
                        //uv
                        0.0,1.0,
                        //normal
                        triangle1_normal.x,triangle1_normal.y,triangle1_normal.z
                    ],
                });
                vertices.push(Vertex {
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x1_y1.x,x1_y1.y,x1_y1.z,
                        //uv
                        1.0,1.0,
                        //normal
                        triangle1_normal.x,triangle1_normal.y,triangle1_normal.z
                    ],
                });
                vertices.push(Vertex {
                    #[rustfmt::skip]
                    data:vec![
                        //position:
                        x1_y0.x,x1_y0.y,x1_y0.z,
                        //uv
                        1.0,0.0,
                        //normal
                        triangle1_normal.x,triangle1_normal.y,triangle1_normal.z
                    ],
                });
            }
        }
        Model {
            mesh: Mesh {
                vertices: vertices.iter().map(|v| v.data.clone()).flatten().collect(),
                description: ItemDesc::default_model(),
            },
            texture: Texture::constant_color(Vector4::new(200, 200, 200, 255), Vector2::new(8, 8)),
            transform,
        }
    }
    pub fn cube(transform: Transform) -> Model {
        let vertices = vec![
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,1.0,
                    //uv
                    1.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //second triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //third triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //fourth triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,-1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //fith triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,-1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    1.0,0.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            //sixth triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    1.0,0.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,-1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,0.0,-1.0
                ],
            },
            //seventh triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,1.0,
                    //uv
                    1.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //eighth triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    -1.0,0.0,0.0
                ],
            },
            //9th triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            //10th triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,-1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,1.0,1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,1.0,0.0
                ],
            },
            //11th triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,1.0,
                    //uv
                    1.0,0.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
            //12th triangle
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,1.0,
                    //uv
                    0.0,0.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    -1.0,-1.0,-1.0,
                    //uv
                    1.0,1.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
            Vertex {
                #[rustfmt::skip]
                data:vec![
                    //position
                    1.0,-1.0,-1.0,
                    //uv
                    0.0,1.0,
                    //normal
                    0.0,-1.0,0.0
                ],
            },
        ];
        Model {
            mesh: Mesh {
                vertices: vertices.iter().map(|v| v.data.clone()).flatten().collect(),
                description: ItemDesc::default_model(),
            },
            texture: Texture::constant_color(Vector4::new(255, 0, 0, 255), Vector2::new(8, 8)),
            transform,
        }
    }
}
