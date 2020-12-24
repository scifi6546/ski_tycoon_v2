use super::prelude::{Mesh, Model, RenderTransform, Texture};
use nalgebra::{Vector2, Vector3, Vector4};
pub fn get_terrain_model(dimensions: Vector2<u32>, transform: RenderTransform) -> Model {
    let mut vertices = vec![];
    let scale = 1.0;
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            let pos = Vector3::new(x as f32, 0.0, y as f32);
            //first traigne
            vertices.push((
                scale * (Vector3::new(0.0, 0.0, 0.0) + pos),
                Vector2::new(0.0, 0.0),
            ));
            vertices.push((
                scale * (Vector3::new(1.0, 0.0, 1.0) + pos),
                Vector2::new(1.0, 1.0),
            ));
            vertices.push((
                scale * (Vector3::new(1.0, 0.0, 0.0) + pos),
                Vector2::new(1.0, 0.0),
            ));
            //second triangle
            vertices.push((
                scale * (Vector3::new(0.0, 0.0, 0.0) + pos),
                Vector2::new(0.0, 0.0),
            ));
            vertices.push((
                scale * (Vector3::new(0.0, 0.0, 1.0) + pos),
                Vector2::new(0.0, 1.0),
            ));
            vertices.push((
                scale * (Vector3::new(1.0, 0.0, 1.0) + pos),
                Vector2::new(1.0, 1.0),
            ));
        }
    }
    Model {
        mesh: Mesh { vertices },
        texture: Texture::constant_color(Vector4::new(0, 255, 255, 255), Vector2::new(8, 8)),
        transform,
    }
}
pub fn get_cube(transform: RenderTransform) -> Model {
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
