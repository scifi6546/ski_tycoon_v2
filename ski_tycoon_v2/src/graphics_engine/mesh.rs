use std::io::Cursor;
use tobj::{load_mtl_buf, load_obj_buf};
#[derive(Clone, Debug)]
pub struct Vertex {
    pub data: Vec<f32>,
}
/// Description of certicies for a mesh
#[derive(Clone, Debug)]
pub struct ItemDesc {
    pub number_components: usize,
    pub size_component: usize,
    pub name: String,
}
impl ItemDesc {
    /// Returns default for a model
    pub fn default_model() -> Vec<ItemDesc> {
        vec![
            ItemDesc {
                number_components: 3,
                size_component: std::mem::size_of::<f32>(),
                name: "position".to_string(),
            },
            ItemDesc {
                number_components: 2,
                size_component: std::mem::size_of::<f32>(),
                name: "uv".to_string(),
            },
            ItemDesc {
                number_components: 3,
                size_component: std::mem::size_of::<f32>(),
                name: "normal".to_string(),
            },
        ]
    }
}
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    //for now description only covers extra items
    pub description: Vec<ItemDesc>,
}
impl Mesh {
    /// gets number of vertices
    pub fn num_vertices(&self) -> usize {
        let vertex_size: usize = self.description.iter().map(|d| d.number_components).sum();
        self.vertices.len() / vertex_size
    }

    /// gets size of vertex in bytes
    pub fn vertex_size(&self) -> usize {
        self.description
            .iter()
            .map(|d| d.number_components * d.size_component)
            .sum()
    }
    /// Loads mesh from an :bj buff
    pub fn from_obj(obj: &'static [u8], mtl: &'static [u8]) -> Self {
        let mut obj_buff = Cursor::new(obj);
        let loaded_obj =
            &(load_obj_buf(&mut obj_buff, true, |_| load_mtl_buf(&mut Cursor::new(mtl)))
                .expect("loaded obj")
                .0)[0]
                .mesh;
        let vertices: Vec<f32> = loaded_obj
            .indices
            .iter()
            .map(|i| {
                vec![
                    loaded_obj.positions[*i as usize * 3],
                    loaded_obj.positions[*i as usize * 3 + 1],
                    loaded_obj.positions[*i as usize * 3 + 2],
                    loaded_obj.texcoords[*i as usize * 2],
                    loaded_obj.texcoords[*i as usize * 2 + 1],
                    loaded_obj.normals[*i as usize],
                    loaded_obj.normals[*i as usize + 1],
                    loaded_obj.normals[*i as usize + 2],
                ]
            })
            .flatten()
            .collect();
        let description = ItemDesc::default_model();
        Self {
            vertices,
            description,
        }
    }
    /// Returns a 2x2 plane aligended with the x-y plane centerd at (0,0,0)
    pub fn plane() -> Self {
        Self {
            #[rustfmt::skip]
            vertices: vec![
                -1.0,-1.0,-0.5,
                0.0,0.0,
                0.0,0.0,1.0,

                1.0,-1.0,-0.5,
                1.0,0.0,
                0.0,0.0,1.0,

                1.0,1.0,-0.5,
                1.0,1.0,
                0.0,0.0,1.0,

                -1.0,-1.0,-0.5,
                0.0,0.0,
                0.0,0.0,1.0,

                -1.0,1.0,-0.5,
                0.0,1.0,
                0.0,0.0,1.0,

                1.0,1.0,-0.5,
                1.0,1.0,
                0.0,0.0,1.0,
            ],
            description: ItemDesc::default_model(),
        }
    }
    pub fn to_bytes(&self) -> Vec<f32> {
        self.vertices.clone()
    }
}
