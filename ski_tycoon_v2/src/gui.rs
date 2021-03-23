pub use super::prelude;
use super::prelude::{
    ErrorType, Event, Mesh, Model, RenderingContext, RuntimeModel, Shader, ShaderBind, Texture,
    Transform,
};
use legion::*;
mod egui_integration;
use egui::CtxRef;
use egui_integration::draw_egui;
pub use egui_integration::EguiRawInputAdaptor;
use nalgebra::{Vector2, Vector4};
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
        webgl: &mut RenderingContext,
        bound_shader: &Shader,
    ) -> Result<Entity, ErrorType> {
        let transform = GuiTransform {
            transform: self.model.transform.clone(),
        };
        let model = RuntimeModel::new(&self.model, webgl, bound_shader)?;
        Ok(world.push((transform, GuiRuntimeModel { model })))
    }
}
trait App: std::fmt::Debug {}
#[derive(Debug, Clone)]
struct Test<T: Clone> {
    t: T,
}
#[allow(unused_must_use)]
pub fn init_gui(screen_size: Vector2<u32>) -> (CtxRef, EguiRawInputAdaptor) {
    let mut ctx = CtxRef::default();
    let mut adaptor = EguiRawInputAdaptor::default();
    ctx.begin_frame(adaptor.process_events(&vec![], screen_size));
    //not painting because it is just in the init phase
    ctx.end_frame();
    (ctx, adaptor)
}
#[allow(clippy::ptr_arg)]
pub fn draw_gui(
    context: &mut CtxRef,
    input: &Vec<Event>,
    gl: &mut RenderingContext,
    shader: &mut ShaderBind,
    adaptor: &mut EguiRawInputAdaptor,
    screen_size: Vector2<u32>,
) -> Result<(), ErrorType> {
    context.begin_frame(adaptor.process_events(input, screen_size));
    let (_, commands) = context.end_frame();
    let paint_jobs = context.tesselate(commands);
    draw_egui(&paint_jobs, &context.texture(), gl, shader, &screen_size)?;
    Ok(())
}
