pub use super::prelude;
use super::prelude::{
    ErrorType, Event, Mesh, Model, RuntimeModel, Shader, ShaderBind, Texture, Transform, WebGl,
};
use legion::*;
use log::info;
mod egui_integration;
use egui::{CtxRef, Ui};
use egui_integration::draw_egui;
pub use egui_integration::EguiRawInputAdaptor;
use epi::App as EpiApp;
use nalgebra::{Vector2, Vector4};
use std::sync::Arc;
pub struct GuiRuntimeModel {
    pub model: RuntimeModel,
}
pub struct GuiTransform {
    pub transform: Transform,
}
#[derive(Clone)]
pub struct GuiModel {
    model: Model,
}
impl GuiModel {
    pub fn simple_box(transform: Transform) -> Self {
        Self {
            model: Model {
                mesh: Mesh::plane(),
                texture: Texture::constant_color(
                    Vector4::new(255 / 10, 255 / 2, 255 / 2, 255),
                    Vector2::new(100, 100),
                ),
                transform,
            },
        }
    }
    pub fn insert(
        &self,
        world: &mut World,
        webgl: &mut WebGl,
        bound_shader: &Shader,
    ) -> Result<Entity, ErrorType> {
        let transform = GuiTransform {
            transform: self.model.transform.clone(),
        };
        let model = RuntimeModel::new(self.model.clone(), webgl, bound_shader)?;
        Ok(world.push((transform, GuiRuntimeModel { model })))
    }
}
trait App: std::fmt::Debug {}
#[derive(Debug, Clone)]
struct Test<T: Clone> {
    t: T,
}
pub fn init_gui() -> (CtxRef, EguiRawInputAdaptor) {
    let mut ctx = CtxRef::default();
    let mut adaptor = EguiRawInputAdaptor::default();
    ctx.begin_frame(adaptor.process_events(&vec![]));
    ctx.end_frame();
    (ctx, adaptor)
}
pub fn draw_gui(
    context: &mut CtxRef,
    input: &Vec<Event>,
    gl: &mut WebGl,
    shader: &mut ShaderBind,
    adaptor: &mut EguiRawInputAdaptor,
) -> Result<(), ErrorType> {
    context.begin_frame(adaptor.process_events(input));
    let (_, commands) = context.end_frame();
    let paint_jobs = context.tesselate(commands);
    draw_egui(&paint_jobs, &context.texture(), gl, shader)?;
    Ok(())
}
pub fn insert_ui(context: &mut CtxRef) {
    egui::Window::new("dfsadfas").show(context, |ui| {
        ui.label("Can I Read?");
    });
}
