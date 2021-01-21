use super::prelude::{
    dijkstra, FollowPath, GraphLayer, GraphLayerList, JsValue, Model, Node, RuntimeModel,
    ShaderBind, Transform, WebGl,
};
mod behavior_tree;
use egui::CtxRef;
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
    let path = dijkstra(
        Node { node: position },
        Node { node: end },
        GraphLayerList::new(layers),
    );
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    let model = Model::cube(transform.clone());
    let runtime_model = RuntimeModel::new(&model, graphics, bound_shader.get_bind())?;
    let follow: FollowPath = FollowPath::new(path);
    world.push((transform, follow, runtime_model));
    info!("built path");
    Ok(())
}

pub fn draw_skiiers(world: &World, context: &mut CtxRef) {
    egui::Window::new("skiiers").show(context, |ui| {
        let mut query = <&FollowPath>::query();
        for path in query.iter(world) {
            ui.collapsing("skiier", |ui| {
                for (node, weight) in path.path.path.iter() {
                    ui.label(format!("{}: {}", node, weight));
                }
            });
            ui.label("skiier");
        }
    });
}
#[system(for_each)]
pub fn follow_path(transform: &mut Transform, path: &mut FollowPath) {
    path.incr(0.01);

    let t = path.get();
    transform.set_translation(Vector3::new(t.node.x as f32, 0.0, t.node.y as f32));
}
