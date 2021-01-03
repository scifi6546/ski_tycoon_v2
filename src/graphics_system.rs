use super::prelude::{
    Camera, ErrorType, GuiRuntimeModel, GuiTransform, Mesh, Model, RuntimeMesh, RuntimeTexture,
    Shader, ShaderBind, Terrain, Transform, WebGl,
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
    pub fn new(
        model: Model,
        graphics: &mut WebGl,
        bound_shader: &Shader,
    ) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(model.mesh, bound_shader)?;
        let texture = graphics.build_texture(model.texture, bound_shader)?;
        Ok(Self { mesh, texture })
    }
}
impl RuntimeDebugMesh {
    pub fn new(mesh: Mesh, graphics: &mut WebGl, bound_shader: &Shader) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(mesh, bound_shader)?;
        Ok(Self { mesh })
    }
}
pub fn insert_terrain(
    terrain: Terrain,
    world: &mut World,
    graphics: &mut WebGl,
    bound_shader: &Shader,
) -> Result<(), ErrorType> {
    let model = terrain.model();
    world.push((
        terrain.build_graph(),
        terrain,
        model.transform.clone(),
        RuntimeDebugMesh::new(model.mesh.clone(), graphics, bound_shader)?,
        RuntimeModel::new(model, graphics, bound_shader)?,
    ));
    Ok(())
}

#[system(for_each)]
pub fn render_object(
    transform: &Transform,
    model: &RuntimeModel,
    #[resource] webgl: &mut WebGl,
    #[resource] shader: &ShaderBind,
    #[resource] camera: &Camera,
) {
    debug!("running render object");
    webgl.bind_texture(&model.texture, shader.get_bind());
    webgl.send_view_matrix(camera.get_matrix(), shader.get_bind());
    webgl.send_model_matrix(transform.build().clone(), shader.get_bind());
    webgl.draw_mesh(&model.mesh);
}
#[system(for_each)]
pub fn render_debug(
    transform: &Transform,
    model: &RuntimeDebugMesh,
    #[resource] webgl: &mut WebGl,
    #[resource] shader: &ShaderBind,
    #[resource] camera: &Camera,
) {
    webgl.send_model_matrix(transform.build().clone(), shader.get_bind());
    webgl.send_view_matrix(camera.get_matrix(), shader.get_bind());
    webgl.draw_lines(&model.mesh);
}
#[system(for_each)]
pub fn render_gui(
    transform: &GuiTransform,
    model: &GuiRuntimeModel,
    #[resource] webgl: &mut WebGl,
    #[resource] shader: &ShaderBind,
) {
    debug!("running render object");
    webgl.bind_texture(&model.model.texture, shader.get_bind());
    webgl.send_model_matrix(transform.transform.build().clone(), shader.get_bind());
    webgl.draw_mesh(&model.model.mesh);
}
