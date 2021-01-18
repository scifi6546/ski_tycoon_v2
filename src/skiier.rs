use super::prelude::{
    dijkstra, find_best_path, FollowPath, GraphLayer, GraphLayerList, JsValue, Model, Node, Path,
    RuntimeModel, ShaderBind, Transform, WebGl,
};
mod behavior_tree;
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3};
pub fn build_skiier(
    world: &mut World,
    graphics: &mut WebGl,
    bound_shader: &ShaderBind,
    position: Vector2<i64>,
    end: Vector2<i64>,
) -> Result<(), JsValue> {
    let layers: Vec<&GraphLayer> = <&GraphLayer>::query().iter(world).collect();
    let path = find_best_path(
        Node { node: position },
        Node { node: end },
        10,
        GraphLayerList::new(layers),
    );
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    let model = Model::cube(transform.clone());
    let runtime_model = RuntimeModel::new(model, graphics, bound_shader.get_bind())?;
    let follow: FollowPath = FollowPath::new(path);
    world.push((transform, follow, runtime_model));
    info!("built path");
    Ok(())
}
#[system(for_each)]
pub fn follow_path(transform: &mut Transform, path: &mut FollowPath) {
    path.incr(0.01);

    let t = path.get();
    transform.set_translation(Vector3::new(t.node.x as f32, 0.0, t.node.y as f32));
}
