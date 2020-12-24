use super::prelude::{
    Camera, ErrorType, GuiRuntimeModel, GuiTransform, Model, RenderTransform, RuntimeMesh,
    RuntimeTexture, WebGl,
};
use legion::*;
use log::debug;
pub struct RuntimeModel {
    mesh: RuntimeMesh,
    texture: RuntimeTexture,
}
impl RuntimeModel {
    pub fn new(model: Model, graphics: &mut WebGl) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(model.mesh)?;
        let texture = graphics.build_texture(model.texture)?;
        Ok(Self { mesh, texture })
    }
}
pub fn insert_mesh(model: Model, world: &mut World, graphics: &mut WebGl) -> Result<(), ErrorType> {
    world.push((model.transform.clone(), RuntimeModel::new(model, graphics)?));
    Ok(())
}
#[system(for_each)]
pub fn render_object(
    transform: &RenderTransform,
    model: &RuntimeModel,
    #[resource] webgl: &mut WebGl,
    #[resource] camera: &Camera,
) {
    debug!("running render object");
    webgl.bind_texture(&model.texture);
    webgl.send_view_matrix(camera.get_matrix());
    webgl.send_model_matrix(transform.get_matrix().clone());
    webgl.draw_mesh(&model.mesh);
}

#[system(for_each)]
pub fn render_gui(
    transform: &GuiTransform,
    model: &GuiRuntimeModel,
    #[resource] webgl: &mut WebGl,
    #[resource] camera: &Camera,
) {
    debug!("running render object");
    webgl.bind_texture(&model.model.texture);
    webgl.send_view_matrix(camera.get_matrix());
    webgl.send_model_matrix(transform.transform.get_matrix().clone());
    webgl.draw_mesh(&model.model.mesh);
}
