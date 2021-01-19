use super::prelude::{
    AssetManager, JsValue, Model, RuntimeModel, ShaderBind, Texture, Transform, WebGl,
};
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3, Vector4};
pub fn insert_lift(
    world: &mut World,
    graphics: &mut WebGl,
    asset_manager: &mut AssetManager<RuntimeModel>,
    bound_shader: &ShaderBind,
    position: Vector2<i64>,
) -> Result<(), JsValue> {
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
    transform.set_translation(Vector3::new(position.x as f32, 0.0, position.y as f32));
    let runtime_model = if asset_manager.contains("lift") {
        asset_manager.get("lift").unwrap().clone()
    } else {
        let model = Model::from_obj(
            include_bytes!["../assets/obj/skilift.obj"],
            include_bytes!["../assets/obj/skilift.obj"],
            transform.clone(),
            Texture::constant_color(Vector4::new(255, 255, 0, 255), Vector2::new(10, 10)),
        );
        asset_manager
            .get_or_create(
                "lift",
                RuntimeModel::new(&model, graphics, bound_shader.get_bind())?,
            )
            .clone()
    };
    world.push((transform, runtime_model));
    info!("built path");
    Ok(())
}
