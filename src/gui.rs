use super::prelude::{
    ErrorType, Mesh, Model, RuntimeModel, Shader, Texture, Transform, Vertex, WebGl,
};
use legion::*;
use nalgebra::{Vector2, Vector3, Vector4};
pub struct GuiRuntimeModel {
    pub model: RuntimeModel,
}
pub struct GuiTransform {
    pub transform: Transform,
}
#[derive(Clone)]
pub struct GuiModel {
    model: Model,
}
impl GuiModel {
    pub fn simple_box(transform: Transform) -> Self {
        Self {
            model: Model {
                mesh: Mesh::plane(),
                texture: Texture::constant_color(
                    Vector4::new(255 / 10, 255 / 2, 255 / 2, 255),
                    Vector2::new(100, 100),
                ),
                transform,
            },
        }
    }
    pub fn insert(
        &self,
        world: &mut World,
        webgl: &mut WebGl,
        bound_shader: &Shader,
    ) -> Result<Entity, ErrorType> {
        let transform = GuiTransform {
            transform: self.model.transform.clone(),
        };
        let model = RuntimeModel::new(self.model.clone(), webgl, bound_shader)?;
        Ok(world.push((transform, GuiRuntimeModel { model })))
    }
}
