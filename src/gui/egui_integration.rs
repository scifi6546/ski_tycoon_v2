use super::prelude::{
    ErrorType, Event, ItemDesc, Mesh, ShaderBind, Texture as RGBATexture, Vertex, WebGl,
};
use egui::{
    math::{Pos2, Rect, Vec2},
    paint::tessellator::Vertex as EguiVertex,
    PaintJobs, RawInput, Texture,
};
use log::info;
use nalgebra::{Vector2, Vector3, Vector4};
use std::sync::Arc;
/// Struct used to get state
///
pub struct EguiRawInputAdaptor {
    is_rightclick_down: bool,
    last_cursor_pos: Vector2<f32>,
    frame_scroll: f32,
}
impl EguiRawInputAdaptor {
    pub fn process_events(&mut self, events: &Vec<Event>, screen_size: Vector2<u32>) -> RawInput {
        self.frame_scroll = 0.0;
        for e in events.iter() {
            match e {
                Event::MouseDown { .. } => {
                    info!("mouse down");
                    self.is_rightclick_down = true;
                    info!("right click down: {}", self.is_rightclick_down);
                }
                Event::MouseUp { .. } => {
                    info!("mouse up ujjj");
                    info!("before??");
                    self.is_rightclick_down = false;
                    info!("right click down: {}", self.is_rightclick_down);
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
            max: Pos2::new(screen_size.x as f32, screen_size.y as f32),
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
    screen_size: &Vector2<u32>,
) -> Result<(), ErrorType> {
    let pixels = texture
        .srgba_pixels()
        .map(|p| Vector4::new(p.r(), p.g(), p.b(), p.a()))
        .collect();
    let dimensions = Vector2::new(texture.width as u32, texture.height as u32);
    let texture = RGBATexture { pixels, dimensions };
    let mut render_texture = gl.build_texture(texture, shader.get_bind())?;
    gl.bind_texture(&render_texture, shader.get_bind());
    let mut depth = -0.8;
    for (_rect, triangles) in paint_jobs.iter() {
        let vertices = to_vertex(
            &triangles
                .indices
                .iter()
                .map(|i| triangles.vertices[*i as usize])
                .collect(),
            depth,
            screen_size,
        );
        let mut runtime_mesh = gl.build_mesh(
            Mesh {
                vertices,
                description: vec![ItemDesc {
                    number_components: 4,
                    size_component: std::mem::size_of::<f32>(),
                    name: "vertex_color".to_string(),
                }],
            },
            shader.get_bind(),
        )?;
        gl.draw_mesh(&runtime_mesh);
        gl.delete_mesh(&mut runtime_mesh)?;
        depth -= 0.01;
    }
    gl.delete_texture(&mut render_texture);
    Ok(())
}
fn to_vertex(vertex_list: &Vec<EguiVertex>, depth: f32, screen_size: &Vector2<u32>) -> Vec<Vertex> {
    let mut vertices = vec![];
    let screen_x = screen_size.x as f32 / 2.0;
    let screen_y = screen_size.y as f32 / 2.0;
    for vertex in vertex_list.iter() {
        let position = Vector3::new(
            vertex.pos.x / screen_x - 1.0,
            -1.0 * vertex.pos.y / screen_y + 1.0,
            depth,
        );
        let uv = Vector2::new(vertex.uv.x, vertex.uv.y);
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let color: egui::paint::Rgba = vertex.color.into();
        let extra_custom = vec![color.r(), color.g(), color.b(), color.a()];
        vertices.push(Vertex {
            position,
            uv,
            normal,
            extra_custom,
        });
    }
    vertices
}
