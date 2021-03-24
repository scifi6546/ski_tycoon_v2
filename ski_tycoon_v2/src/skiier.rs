use super::prelude::{
    AssetManager, ErrorType, FollowPath, GraphLayer, GraphLayerList, Model, Node, Path,
    RenderingContext, RuntimeModel, RuntimeModelId, ShaderBind, Terrain, Transform,
};
mod behavior_tree;
use behavior_tree::{Number, SearchStart, TreeNode};
use egui::CtxRef;
use legion::*;
use nalgebra::{Vector2, Vector3};
pub struct DecisionDebugInfo {
    name: String,
    cost: Number<f32>,
    start: Node,
    end: Node,
    path_len: usize,
}
#[allow(clippy::ptr_arg)]
fn run_skiier_ai(
    layers: &Vec<&GraphLayer>,
    start_position: Vector2<i64>,
    terrain: &Terrain,
) -> (FollowPath, Vec<DecisionDebugInfo>) {
    let tree_start: Box<dyn TreeNode> = Box::new(SearchStart::default());
    let decisions = tree_start.best_path(
        4,
        &GraphLayerList::new(layers.clone()),
        Node {
            node: start_position,
        },
        terrain,
    );
    let follow = decisions
        .iter()
        .fold(FollowPath::new(Path::default(), terrain), |acc, x| {
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
    (follow, decision_debug_info)
}
pub fn build_skiier(
    world: &mut World,
    graphics: &mut RenderingContext,

    asset_manager: &mut AssetManager<RuntimeModel>,
    bound_shader: &ShaderBind,
    position: Vector2<i64>,
) -> Result<(), ErrorType> {
    let layers: Vec<&GraphLayer> = <&GraphLayer>::query().iter(world).collect();
    let terrain = <&Terrain>::query().iter(world).next().unwrap();
    let (follow, decision_debug_info) = run_skiier_ai(&layers, position, terrain);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
    if !asset_manager.contains("skiier") {
        let model = Model::cube(transform.clone());
        asset_manager.get_or_create(
            "skiier",
            RuntimeModel::new(&model, graphics, bound_shader.get_bind())
                .expect("failed to build run time model"),
        );
    }
    let runtime_model = RuntimeModelId::new("skiier".to_string());

    world.push((transform, follow, runtime_model, decision_debug_info));
    Ok(())
}

pub fn draw_skiiers(world: &World, context: &mut CtxRef) {
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
pub fn follow_path(world: &mut World) {
    let layers: Vec<GraphLayer> = <&GraphLayer>::query().iter(world).cloned().collect();
    //bad hack inorder to avoid copying terrain more then needed
    let terrain_iter = <&Terrain>::query().iter(world).next();
    if terrain_iter.is_none() {
        return;
    }
    let terrain: *const Terrain = &*terrain_iter.unwrap();
    let borrow_graph_layer = layers.iter().collect();
    let mut query = <(&mut Transform, &mut FollowPath, &mut Vec<DecisionDebugInfo>)>::query();
    for (transform, path, debug_info) in query.iter_mut(world) {
        if path.at_end() {
            if let Some(endpoint) = path.endpoint() {
                let (t_path, t_debug_info) =
                    run_skiier_ai(&borrow_graph_layer, endpoint.node, unsafe { &*terrain });
                if !t_path.is_empty() {
                    let t = path.get();
                    transform.set_translation(t);
                }
                *path = t_path;
                *debug_info = t_debug_info;
            }
        } else {
            path.incr(0.1);
            if !path.is_empty() {
                let t = path.get();
                transform.set_translation(t);
            }
        }
    }
}
