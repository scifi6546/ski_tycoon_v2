use super::prelude::{
    AssetManager, ErrorType, GraphLayer, GraphWeight, LiftLayer, Model, Node, RenderingContext,
    RuntimeModel, RuntimeModelId, ShaderBind, Terrain, Texture, Transform,
};
const SKI_LIFT: &'static [u8] = include_bytes!["../../assets/obj/skilift.obj"];
use egui::CtxRef;
use legion::*;
use log::info;
use nalgebra::{Vector2, Vector3, Vector4};
pub fn insert_lift(
    world: &mut World,
    graphics: &mut RenderingContext,
    asset_manager: &mut AssetManager<RuntimeModel>,

    bound_shader: &ShaderBind,
    start_position: Vector2<i64>,
    end_position: Vector2<i64>,
) -> Result<(), ErrorType> {
    let mut transform = Transform::default();
    let mut end_transform = Transform::default();
    {
        //setting scope to minimize borrow time
        let terrain = <&Terrain>::query().iter(world).next().unwrap();
        transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
        transform.set_translation(terrain.get_transform(&start_position).unwrap());
        end_transform.set_translation(terrain.get_transform(&end_position).unwrap());
    }
    if !asset_manager.contains("lift") {
        let model = Model::from_obj(
            SKI_LIFT,
            SKI_LIFT,
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
    world.push((end_transform, runtime_model));
    info!("built path");
    Ok(())
}
/// Labels a lift as in progress
struct LiftPlace {
    position: Vector2<i64>,
}
impl Default for LiftPlace {
    fn default() -> Self {
        Self {
            position: Vector2::new(0, 0),
        }
    }
}
pub struct BuildLift {
    placing_lift: bool,
}
impl BuildLift {
    pub fn build_lift(
        &mut self,
        world: &mut World,
        context: &mut CtxRef,
        graphics: &mut RenderingContext,
        asset_manager: &mut AssetManager<RuntimeModel>,
        bound_shader: &ShaderBind,
    ) {
        let previous_placing_lift = self.placing_lift.clone();
        let mut direction: Vector2<i64> = Vector2::new(0, 0);
        egui::Window::new("lift").show(context, |ui| {
            let response = ui.button("add lift");
            if response.clicked {
                self.placing_lift = true;
            } else {
                ui.label("not clicked");
            }
            if self.placing_lift {
                if ui.button("x+").clicked {
                    direction.x += 1;
                }
                if ui.button("x-").clicked {
                    direction.x -= 1;
                }
                if ui.button("y+").clicked {
                    direction.y += 1;
                }
                if ui.button("y-").clicked {
                    direction.y -= 1;
                }
            }
        });
        if previous_placing_lift == false && self.placing_lift {
            let transform = Transform::default();
            if !asset_manager.contains("lift") {
                let model = Model::from_obj(
                    SKI_LIFT,
                    SKI_LIFT,
                    transform.clone(),
                    Texture::constant_color(Vector4::new(255, 255, 0, 255), Vector2::new(10, 10)),
                );
                asset_manager.get_or_create(
                    "lift",
                    RuntimeModel::new(&model, graphics, bound_shader.get_bind())
                        .expect("failed to build run time model"),
                );
            }
            let runtime_model = RuntimeModelId::new("lift".to_string());
            world.push((runtime_model, LiftPlace::default(), transform));
        }
        if direction != Vector2::new(0, 0) && self.placing_lift {
            let mut new_pos = None;
            let mut world_choordinates = Vector3::new(0.0, 0.0, 0.0);
            {
                let lift = <&LiftPlace>::query().iter(world).next().unwrap();
                let terrain = <&Terrain>::query().iter(world).next().unwrap();
                if let Some(p) = terrain.get_transform(&(lift.position + direction)) {
                    world_choordinates = p;
                    new_pos = Some(lift.position + direction);
                }
            }
            if let Some(p) = new_pos {
                let (lift, transform) = <(&mut LiftPlace, &mut Transform)>::query()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                lift.position = p.clone();
                transform.set_translation(world_choordinates);
            }
        }
    }
}
impl Default for BuildLift {
    fn default() -> Self {
        BuildLift {
            placing_lift: false,
        }
    }
}
