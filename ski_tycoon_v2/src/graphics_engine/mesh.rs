use nalgebra::{Vector2, Vector3};
use std::io::Cursor;
use tobj::{load_mtl_buf, load_obj_buf};
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
    /// Loads mesh from an obj buff
    pub fn from_obj(obj: &'static [u8], mtl: &'static [u8]) -> Self {
        let mut obj_buff = Cursor::new(obj);
        let loaded_obj =
            &(load_obj_buf(&mut obj_buff, true, |_| load_mtl_buf(&mut Cursor::new(mtl)))
                .expect("loaded obj")
                .0)[0]
                .mesh;
        let vertices: Vec<Vertex> = loaded_obj
            .indices
            .iter()
            .map(|i| Vertex {
                position: Vector3::new(
                    loaded_obj.positions[*i as usize * 3],
                    loaded_obj.positions[*i as usize * 3 + 1],
                    loaded_obj.positions[*i as usize * 3 + 2],
                ),
                uv: Vector2::new(
                    loaded_obj.texcoords[*i as usize * 2],
                    loaded_obj.texcoords[*i as usize * 2 + 1],
                ),
                normal: Vector3::new(
                    loaded_obj.normals[*i as usize],
                    loaded_obj.normals[*i as usize + 1],
                    loaded_obj.normals[*i as usize + 2],
                ),
                extra_custom: vec![],
            })
            .collect();
        let description = vec![];
        Self {
            vertices,
            description,
        }
    }
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
