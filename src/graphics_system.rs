use super::prelude::{
    Camera, ErrorType, GuiRuntimeModel, GuiTransform, Mesh, Model, RuntimeMesh, RuntimeTexture,
    Terrain, Transform, WebGl,
};
use legion::*;
use log::debug;
pub struct RuntimeModel {
    pub mesh: RuntimeMesh,
    pub texture: RuntimeTexture,
}
/// Used for printing debug info
pub struct RuntimeDebugMesh {
    mesh: RuntimeMesh,
}
impl RuntimeModel {
    pub fn new(model: Model, graphics: &mut WebGl) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(model.mesh)?;
        let texture = graphics.build_texture(model.texture)?;
        Ok(Self { mesh, texture })
    }
}
impl RuntimeDebugMesh {
    pub fn new(mesh: Mesh, graphics: &mut WebGl) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(mesh)?;
        Ok(Self { mesh })
    }
}
pub fn insert_terrain(
    terrain: Terrain,
    world: &mut World,
    graphics: &mut WebGl,
) -> Result<(), ErrorType> {
    let model = terrain.model();
    world.push((
        terrain.build_graph(),
        terrain,
        model.transform.clone(),
        RuntimeDebugMesh::new(model.mesh.clone(), graphics)?,
        RuntimeModel::new(model, graphics)?,
    ));
    Ok(())
}

#[system(for_each)]
pub fn render_object(
    transform: &Transform,
    model: &RuntimeModel,
    #[resource] webgl: &mut WebGl,
    #[resource] camera: &Camera,
) {
    debug!("running render object");
    webgl.bind_texture(&model.texture);
    webgl.send_view_matrix(camera.get_matrix());
    webgl.send_model_matrix(transform.build().clone());
    webgl.draw_mesh(&model.mesh);
}
#[system(for_each)]
pub fn render_debug(
    transform: &Transform,
    model: &RuntimeDebugMesh,
    #[resource] webgl: &mut WebGl,
    #[resource] camera: &Camera,
) {
    webgl.send_model_matrix(transform.build().clone());
    webgl.send_view_matrix(camera.get_matrix());
    webgl.draw_lines(&model.mesh);
}
#[system(for_each)]
pub fn render_gui(
    transform: &GuiTransform,
    model: &GuiRuntimeModel,
    #[resource] webgl: &mut WebGl,
) {
    debug!("running render object");
    webgl.bind_texture(&model.model.texture);
    webgl.send_model_matrix(transform.transform.build().clone());
    webgl.draw_mesh(&model.model.mesh);
}
