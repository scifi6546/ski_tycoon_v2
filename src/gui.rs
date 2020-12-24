use super::prelude::{ErrorType, Mesh, Model, RenderTransform, RuntimeModel, Texture, WebGl};
use legion::*;
use nalgebra::{Vector2, Vector3, Vector4};
pub struct GuiRuntimeModel {
    pub model: RuntimeModel,
}
pub struct GuiTransform {
    pub transform: RenderTransform,
}
#[derive(Clone)]
pub struct GuiModel {
    model: Model,
}
impl GuiModel {
    pub fn simple_box(transform: RenderTransform) -> Self {
        Self {
            model: Model {
                mesh: Mesh {
                    vertices: vec![
                        (Vector3::new(1.0, 1.0, -0.5), Vector2::new(1.0, 1.0)),
                        (Vector3::new(-1.0, -1.0, -0.5), Vector2::new(0.0, 0.0)),
                        (Vector3::new(1.0, -1.0, -0.5), Vector2::new(1.0, 0.0)),
                        //Second Triangle
                        (Vector3::new(1.0, 1.0, -0.5), Vector2::new(1.0, 1.0)),
                        (Vector3::new(-1.0, -1.0, -0.5), Vector2::new(0.0, 0.0)),
                        (Vector3::new(-1.0, 1.0, -0.5), Vector2::new(0.0, 1.0)),
                    ],
                },
                texture: Texture::constant_color(
                    Vector4::new(255 / 10, 255 / 2, 255 / 2, 255),
                    Vector2::new(100, 100),
                ),
                transform,
            },
        }
    }
    pub fn insert(&self, world: &mut World, webgl: &mut WebGl) -> Result<Entity, ErrorType> {
        let transform = GuiTransform {
            transform: self.model.transform.clone(),
        };
        let model = RuntimeModel::new(self.model.clone(), webgl)?;
        Ok(world.push((transform, GuiRuntimeModel { model })))
    }
}
