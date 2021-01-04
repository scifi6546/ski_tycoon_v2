use super::prelude::{Event, MouseButton, WebGl};
use egui::{
    math::{Pos2, Rect, Vec2},
    PaintJobs, RawInput, Texture,
};
use nalgebra::Vector2;
use std::sync::Arc;
/// Struct used to get state
pub struct EguiRawInputAdaptor {
    is_rightclick_down: bool,
    last_cursor_pos: Vector2<f32>,
    frame_scroll: f32,
}
impl EguiRawInputAdaptor {
    pub fn process_events(&mut self, events: Vec<Event>) -> RawInput {
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
pub fn draw_egui(paint_jobs: &PaintJobs, texture: &Arc<Texture>, gl: &mut WebGl) {
    todo!()
}
