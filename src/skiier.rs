use super::prelude::{
    dijkstra, FollowPath, GraphLayer, GraphLayerList, JsValue, Mesh, Model, RuntimeDebugMesh,
    RuntimeModel, Transform, WebGl,
};
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3};
struct f<T> {
    t: T,
}
pub fn build_skiier(
    world: &mut World,
    graphics: &mut WebGl,
    position: Vector2<i64>,
) -> Result<(), JsValue> {
    let layers: Vec<GraphLayer> = <&GraphLayer>::query()
        .iter(world)
        .map(|l| l.clone())
        .collect();
    let path = dijkstra(position, Vector2::new(3, 3), GraphLayerList::new(layers));
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    let model = Model::cube(transform);
    let runtime_model = RuntimeModel::new(model, graphics)?;
    let vertices: Vec<(Vector3<f32>, Vector2<f32>)> = path
        .path
        .iter()
        .map(|v| {
            info!("{} {}", v.x, v.y);
            v
        })
        .map(|v| {
            (
                Vector3::new(v.x as f32, 0.0, v.y as f32),
                Vector2::new(0.0, 0.0),
            )
        })
        .collect();
    let transform = Transform::default();
    let mesh = Mesh { vertices };
    let follow: FollowPath<GraphLayerList> = FollowPath::new(path);
    world.push((
        transform,
        RuntimeDebugMesh::new(mesh, graphics)?,
        follow,
        runtime_model,
    ));
    info!("built path");
    Ok(())
}
#[system(for_each)]
pub fn follow_path(transform: &mut Transform, path: &mut FollowPath<GraphLayerList>) {
    path.incr(0.01);

    let t = path.get();
    transform.set_translation(Vector3::new(t.x as f32, 0.0, t.y as f32));
}
