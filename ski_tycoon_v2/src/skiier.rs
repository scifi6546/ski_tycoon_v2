use super::prelude::{
    ErrorType, FollowPath, GraphLayer, GraphLayerList, Model, Node, Path, RenderingContext,
    RuntimeModel, ShaderBind, Transform,
};
mod behavior_tree;
use behavior_tree::{Number, SearchStart, TreeNode};
use egui::CtxRef;
use legion::*;
use nalgebra::{Vector2, Vector3};
struct DecisionDebugInfo {
    name: String,
    cost: Number<f32>,
    start: Node,
    end: Node,
    path_len: usize,
}
pub fn build_skiier(
    world: &mut World,
    graphics: &mut RenderingContext,
    bound_shader: &ShaderBind,
    position: Vector2<i64>,
) -> Result<(), ErrorType> {
    let tree_start: Box<dyn TreeNode> = Box::new(SearchStart::default());
    let layers: Vec<&GraphLayer> = <&GraphLayer>::query().iter(world).collect();
    let decisions = tree_start.best_path(4, &GraphLayerList::new(layers), Node { node: position });
    let follow = decisions
        .iter()
        .fold(FollowPath::new(Path::default()), |acc, x| {
            acc.append(&x.path)
        });

    let decision_debug_info: Vec<DecisionDebugInfo> = decisions
        .iter()
        .map(|d| DecisionDebugInfo {
            name: d.name.clone(),
            cost: d.cost.clone(),
            path_len: d.path.len(),
            start: if let Some(p) = d.path.start() {
                p.clone()
            } else {
                d.endpoint.clone()
            },
            end: d.endpoint.clone(),
        })
        .collect();
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    let model = Model::cube(transform.clone());
    let runtime_model = RuntimeModel::new(&model, graphics, bound_shader.get_bind())?;
    world.push((transform, follow, runtime_model, decision_debug_info));
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
    egui::Window::new("skiier decisions").show(context, |ui| {
        let mut query = <&Vec<DecisionDebugInfo>>::query();
        for skiier in query.iter(world) {
            ui.collapsing("skiier", |ui| {
                for debug in skiier.iter() {
                    ui.label(format!(
                        "{}: {}, path len: {}, start: {}, end: {}",
                        debug.name, debug.cost, debug.path_len, debug.start, debug.end
                    ));
                }
            });
        }
    });
}
#[system(for_each)]
pub fn follow_path(transform: &mut Transform, path: &mut FollowPath) {
    path.incr(0.01);
    if path.len() > 0 {
        let t = path.get();
        transform.set_translation(Vector3::new(t.node.x as f32, 0.0, t.node.y as f32));
    }
}
