use super::prelude::{
    AssetManager, ErrorType, GraphLayer, GraphWeight, LiftLayer, Model, Node, RenderingContext,
    RuntimeModel, RuntimeModelId, ShaderBind, Terrain, Texture, Transform,
};
const SKI_LIFT: &[u8] = include_bytes!["../../assets/obj/skilift.obj"];
use egui::CtxRef;
use legion::*;
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
    Ok(())
}

/// Labels a lift as in progress
struct FirstLiftPlace {
    position: Vector2<i64>,
}
impl Default for FirstLiftPlace {
    fn default() -> Self {
        Self {
            position: Vector2::new(0, 0),
        }
    }
}
/// Labels a lift as in progress
struct SecondLiftPlace {
    bottom_position: Vector2<i64>,
    position: Vector2<i64>,
}
#[derive(Clone, Debug, PartialEq)]
enum LiftStage {
    NoLift,
    FirstLift,
    SecondLift,
    Done,
}
fn get_skilift(
    graphics: &mut RenderingContext,
    asset_manager: &mut AssetManager<RuntimeModel>,
    bound_shader: &ShaderBind,
    transform: &Transform,
) -> RuntimeModelId {
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
    RuntimeModelId::new("lift".to_string())
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuildLift {
    placing_lift: LiftStage,
    start_lift: Option<Entity>,
    end_lift: Option<Entity>,
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
            ui.label("Add Lift");
            let response = ui.button("add lift");
            if response.clicked {
                self.placing_lift = match self.placing_lift {
                    LiftStage::NoLift => LiftStage::FirstLift,
                    LiftStage::FirstLift => LiftStage::SecondLift,
                    LiftStage::SecondLift => LiftStage::Done,
                    LiftStage::Done => panic!("invalid lift state"),
                }
            }
            if self.placing_lift == LiftStage::FirstLift
                || self.placing_lift == LiftStage::SecondLift
            {
                ui.label("x +");
                if ui.button("x+").clicked {
                    direction.x += 1;
                }
                ui.label("x -");
                if ui.button("x-").clicked {
                    direction.x -= 1;
                }

                ui.label("y +");
                if ui.button("y+").clicked {
                    direction.y += 1;
                }
                ui.label("y -");
                if ui.button("y-").clicked {
                    direction.y -= 1;
                }
            }
        });
        if previous_placing_lift == LiftStage::NoLift && self.placing_lift == LiftStage::FirstLift {
            let transform = Transform::default();
            let runtime_model = get_skilift(graphics, asset_manager, bound_shader, &transform);
            self.start_lift =
                Some(world.push((runtime_model, FirstLiftPlace::default(), transform)));
        }
        if previous_placing_lift == LiftStage::FirstLift
            && self.placing_lift == LiftStage::SecondLift
        {
            let bottom_position = <&FirstLiftPlace>::query()
                .iter(world)
                .next()
                .unwrap()
                .position;
            let transform = Transform::default();
            let runtime_model = get_skilift(graphics, asset_manager, bound_shader, &transform);

            let lift = SecondLiftPlace {
                bottom_position,
                position: Vector2::new(0, 0),
            };
            self.end_lift = Some(world.push((runtime_model, lift, transform)));
        }
        if direction != Vector2::new(0, 0) && self.placing_lift == LiftStage::FirstLift {
            let mut new_pos = None;
            let mut world_choordinates = Vector3::new(0.0, 0.0, 0.0);
            {
                let lift = <&FirstLiftPlace>::query().iter(world).next().unwrap();
                let terrain = <&Terrain>::query().iter(world).next().unwrap();
                if let Some(p) = terrain.get_transform(&(lift.position + direction)) {
                    world_choordinates = p;
                    new_pos = Some(lift.position + direction);
                }
            }
            if let Some(p) = new_pos {
                let (lift, transform) = <(&mut FirstLiftPlace, &mut Transform)>::query()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                lift.position = p;
                transform.set_translation(world_choordinates);
            }
        }
        if direction != Vector2::new(0, 0) && self.placing_lift == LiftStage::SecondLift {
            let mut new_pos = None;
            let mut world_choordinates = Vector3::new(0.0, 0.0, 0.0);
            {
                let lift = <&SecondLiftPlace>::query().iter(world).next().unwrap();
                let terrain = <&Terrain>::query().iter(world).next().unwrap();
                if let Some(p) = terrain.get_transform(&(lift.position + direction)) {
                    world_choordinates = p;
                    new_pos = Some(lift.position + direction);
                }
            }
            if let Some(p) = new_pos {
                let (lift, transform) = <(&mut SecondLiftPlace, &mut Transform)>::query()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                lift.position = p;
                transform.set_translation(world_choordinates);
            }
        }
        if self.placing_lift == LiftStage::Done {
            let lift = <&SecondLiftPlace>::query().iter(world).next().unwrap();
            let bottom_position = lift.bottom_position;
            let top_position = lift.position;
            insert_lift(
                world,
                graphics,
                asset_manager,
                bound_shader,
                bottom_position,
                top_position,
            )
            .expect("failed to insert lift");
            world.remove(self.start_lift.unwrap());
            world.remove(self.end_lift.unwrap());
            self.placing_lift = LiftStage::NoLift;
        }
    }
}
impl Default for BuildLift {
    fn default() -> Self {
        BuildLift {
            placing_lift: LiftStage::NoLift,
            start_lift: None,
            end_lift: None,
        }
    }
}
