use super::prelude::{
    AssetManager, DeltaCamera, ErrorType, GuiRuntimeModel, GuiTransform, Mesh, Model,
    RenderingContext, RuntimeMesh, RuntimeTexture, Shader, ShaderBind, Terrain, Transform,
};
use legion::*;
use log::debug;
use nalgebra::Vector2;
pub struct RuntimeModel {
    pub mesh: RuntimeMesh,
    pub texture: RuntimeTexture,
}
/// Used for printing debug info
pub struct RuntimeDebugMesh {
    mesh: RuntimeMesh,
}
pub struct GraphicsSettings {
    pub screen_size: Vector2<u32>,
}
#[derive(Clone)]
pub struct RuntimeModelId {
    id: String,
}
impl RuntimeModelId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
impl RuntimeModel {
    pub fn new(
        model: &Model,
        graphics: &mut RenderingContext,
        bound_shader: &Shader,
    ) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(model.mesh.clone(), bound_shader)?;
        let texture = graphics.build_texture(model.texture.clone(), bound_shader)?;
        Ok(Self { mesh, texture })
    }
}
impl RuntimeDebugMesh {
    pub fn new(
        mesh: Mesh,
        graphics: &mut RenderingContext,
        bound_shader: &Shader,
    ) -> Result<Self, ErrorType> {
        let mesh = graphics.build_mesh(mesh, bound_shader)?;
        Ok(Self { mesh })
    }
}
pub fn insert_terrain(
    terrain: Terrain,
    world: &mut World,
    graphics: &mut RenderingContext,
    asset_manager: &mut AssetManager<RuntimeModel>,
    bound_shader: &Shader,
) -> Result<(), ErrorType> {
    let model = terrain.model();
    let transform = model.transform.clone();
    asset_manager.overwrite(
        "game_terrain",
        RuntimeModel::new(&model, graphics, bound_shader).expect("created model"),
    );
    world.push((
        terrain.build_graph(),
        terrain,
        transform,
        RuntimeModelId {
            id: "game_terrain".to_string(),
        },
    ));
    Ok(())
}

#[system(for_each)]
pub fn render_object(
    transform: &Transform,
    model: &RuntimeModelId,
    #[resource] settings: &GraphicsSettings,
    #[resource] webgl: &mut RenderingContext,
    #[resource] shader: &ShaderBind,
    #[resource] camera: &DeltaCamera,
    #[resource] asset_manager: &mut AssetManager<RuntimeModel>,
) {
    debug!("running render object");
    let model = asset_manager.get(&model.id).unwrap();
    webgl.bind_texture(&model.texture, shader.get_bind());
    webgl.send_view_matrix(camera.get_matrix(settings.screen_size), shader.get_bind());
    webgl.send_model_matrix(transform.build(), shader.get_bind());
    webgl.draw_mesh(&model.mesh);
}
#[system(for_each)]
pub fn render_debug(
    transform: &Transform,
    model: &RuntimeDebugMesh,
    #[resource] settings: &GraphicsSettings,
    #[resource] webgl: &mut RenderingContext,
    #[resource] shader: &ShaderBind,
    #[resource] camera: &DeltaCamera,
) {
    webgl.send_model_matrix(transform.build(), shader.get_bind());
    webgl.send_view_matrix(camera.get_matrix(settings.screen_size), shader.get_bind());
    webgl.draw_lines(&model.mesh);
}
#[system(for_each)]
pub fn render_gui(
    transform: &GuiTransform,
    model: &GuiRuntimeModel,
    #[resource] webgl: &mut RenderingContext,
    #[resource] shader: &ShaderBind,
) {
    debug!("running render object");
    webgl.bind_texture(&model.model.texture, shader.get_bind());
    webgl.send_model_matrix(transform.transform.build(), shader.get_bind());
    webgl.draw_mesh(&model.model.mesh);
}
