use super::prelude::{
    AssetManager, ErrorType, GraphLayer, GraphWeight, LiftLayer, Model, Node, RenderingContext,
    RuntimeModel, RuntimeModelId, ShaderBind, Texture, Transform,
};
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3, Vector4};
use std::{cell::RefCell, sync::RwLock};
pub fn insert_lift(
    world: &mut World,
    graphics: &mut RenderingContext,
    asset_manager: &mut AssetManager<RuntimeModel>,
    bound_shader: &ShaderBind,
    start_position: Vector2<i64>,
    end_position: Vector2<i64>,
) -> Result<(), ErrorType> {
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
    transform.set_translation(Vector3::new(
        start_position.x as f32,
        0.0,
        start_position.y as f32,
    ));
    if !asset_manager.contains("lift") {
        let model = Model::from_obj(
            include_bytes!["../../assets/obj/skilift.obj"],
            include_bytes!["../../assets/obj/skilift.obj"],
            transform.clone(),
            Texture::constant_color(Vector4::new(255, 255, 0, 255), Vector2::new(10, 10)),
        );
        asset_manager.get_or_create(
            "lift",
            RuntimeModel::new(&model, graphics, bound_shader.get_bind())?,
        );
    }
    let runtime_model = RuntimeModelId::new("lift".to_string());
    let start = Node {
        node: start_position,
    };
    let end = Node { node: end_position };
    world.push((
        transform,
        runtime_model.clone(),
        GraphLayer::Lift(LiftLayer {
            start,
            end,
            weight: GraphWeight::Some(1),
        }),
    ));
    let mut end_transform = Transform::default();
    end_transform.set_translation(Vector3::new(
        end_position.x as f32,
        0.0,
        end_position.y as f32,
    ));
    world.push((end_transform, runtime_model));
    info!("built path");
    Ok(())
}
