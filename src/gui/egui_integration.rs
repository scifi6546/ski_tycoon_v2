use super::prelude::{
    ErrorType, Event, Mesh, MouseButton, ShaderBind, Texture as RGBATexture, Vertex, WebGl,
};
use egui::{
    math::{Pos2, Rect, Vec2},
    paint::tessellator::Vertex as EguiVertex,
    PaintJobs, RawInput, Texture,
};
use log::info;
use nalgebra::{Vector2, Vector3, Vector4};
use std::sync::Arc;
/// Struct used to get statell
///
pub struct EguiRawInputAdaptor {
    is_rightclick_down: bool,
    last_cursor_pos: Vector2<f32>,
    frame_scroll: f32,
}
impl EguiRawInputAdaptor {
    pub fn process_events(&mut self, events: &Vec<Event>) -> RawInput {
        self.frame_scroll = 0.0;
        for e in events.iter() {
            match e {
                Event::MouseDown { button, .. } => {
                    if button == &MouseButton::LeftClick {
                        self.is_rightclick_down = true;
                    }
                }
                Event::MouseUp { button, .. } => {
                    if button == &MouseButton::LeftClick {
                        self.is_rightclick_down = false;
                    }
                }
                Event::MouseMove { x, y, .. } => {
                    self.last_cursor_pos = Vector2::new(x.clone(), y.clone())
                }
                Event::Scroll { delta_y, .. } => self.frame_scroll += delta_y,
                _ => (),
            }
        }
        let mut input = RawInput::default();
        input.mouse_down = self.is_rightclick_down;
        input.mouse_pos = Some(Pos2::new(self.last_cursor_pos.x, self.last_cursor_pos.y));
        input.scroll_delta = Vec2::new(0.0, self.frame_scroll);
        input.screen_rect = Some(Rect {
            min: Pos2::new(0.0, 0.0),
            max: Pos2::new(800.0, 800.0),
        });
        return input;
    }
}
impl Default for EguiRawInputAdaptor {
    fn default() -> Self {
        Self {
            is_rightclick_down: false,
            last_cursor_pos: Vector2::new(0.0, 0.0),
            frame_scroll: 0.0,
        }
    }
}
pub fn draw_egui(
    paint_jobs: &PaintJobs,
    texture: &Arc<Texture>,
    gl: &mut WebGl,
    shader: &ShaderBind,
) -> Result<(), ErrorType> {
    let pixels = texture
        .srgba_pixels()
        .map(|p| Vector4::new(p.r(), p.g(), p.b(), p.a()))
        .collect();
    let dimensions = Vector2::new(texture.width as u32, texture.height as u32);
    let texture = RGBATexture { pixels, dimensions };
    let mut render_texture = gl.build_texture(texture, shader.get_bind())?;
    gl.bind_texture(&render_texture, shader.get_bind());
    for (_rect, triangles) in paint_jobs.iter() {
        let vertices: Vec<Vertex> = triangles
            .indices
            .iter()
            .map(|i| triangles.vertices[*i as usize])
            //.map(|v| {
            //    info!("{:?}", v);
            //    v
            //})
            .map(|v| to_vertex(&v))
            //.map(|i| {
            //    info!("{:?}", i);
            //    i
            //})
            .collect();
        info!("vertex len: {}", vertices.len());
        let mut runtime_mesh = gl.build_mesh(Mesh { vertices }, shader.get_bind())?;
        gl.draw_mesh(&runtime_mesh);
        gl.delete_mesh(&mut runtime_mesh)?;
    }
    gl.delete_texture(&mut render_texture);
    Ok(())
}
fn to_vertex(vertex: &EguiVertex) -> Vertex {
    let position = Vector3::new(
        vertex.pos.x / 400.0 - 1.0,
        -1.0 * vertex.pos.y / 400.0 + 1.0,
        -0.8,
    );
    let uv = Vector2::new(vertex.uv.x, vertex.uv.y);
    let normal = Vector3::new(0.0, 0.0, 1.0);
    Vertex {
        position,
        uv,
        normal,
    }
}
