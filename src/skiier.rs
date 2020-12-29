use super::prelude::{
    dijkstra, GraphLayer, GraphLayerList, JsValue, Mesh, Model, RuntimeDebugMesh, RuntimeModel,
    Transform, WebGl,
};
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3};
pub fn build_skiier(
    world: &mut World,
    graphics: &mut WebGl,
    position: Vector2<i64>,
) -> Result<(), JsValue> {
    let layers: Vec<&GraphLayer> = <&GraphLayer>::query().iter(world).collect();
    let path = dijkstra(position, Vector2::new(10, 10), GraphLayerList::new(layers));
    let model = Model::cube(Transform::default());
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
    world.push((
        transform,
        RuntimeDebugMesh::new(mesh, graphics)?,
        runtime_model,
    ));
    info!("built path");
    Ok(())
}
