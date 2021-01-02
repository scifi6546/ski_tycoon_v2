use super::prelude::{
    dijkstra, FollowPath, GraphLayer, GraphLayerList, JsValue, Model, RuntimeModel, Shader,
    Transform, WebGl,
};
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3};
pub fn build_skiier(
    world: &mut World,
    graphics: &mut WebGl,
    bound_shader: &Shader,
    position: Vector2<i64>,
    end: Vector2<i64>,
) -> Result<(), JsValue> {
    let layers: Vec<GraphLayer> = <&GraphLayer>::query()
        .iter(world)
        .map(|l| l.clone())
        .collect();
    let path = dijkstra(position, end, GraphLayerList::new(layers));
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    let model = Model::cube(transform.clone());
    let runtime_model = RuntimeModel::new(model, graphics, bound_shader)?;
    let follow: FollowPath<GraphLayerList> = FollowPath::new(path);
    world.push((transform, follow, runtime_model));
    info!("built path");
    Ok(())
}
#[system(for_each)]
pub fn follow_path(transform: &mut Transform, path: &mut FollowPath<GraphLayerList>) {
    path.incr(0.01);

    let t = path.get();
    transform.set_translation(Vector3::new(t.x as f32, 0.0, t.y as f32));
}
