use js_sys::{Array as JsArray, Map as JsMap};
use nalgebra::{Matrix4, Vector2, Vector3};
mod events;
use events::MouseButton;
use wasm_bindgen::prelude::*;
#[derive(Clone, Debug)]
pub struct RenderTransform {
    matrix: Matrix4<f32>,
}
impl RenderTransform {
    pub fn new_scale(scale: &Vector3<f32>) -> Self {
        Self {
            matrix: Matrix4::new_nonuniform_scaling(scale),
        }
    }
}
#[derive(Clone)]
pub struct MouseClick {
    position: Vector2<f32>,
    button_pressed: MouseButton,
}
#[derive(Clone)]
pub enum Event {
    MouseMove {
        delta_x: f32,
        delta_y: f32,
        delta_time_ms: f32,
        buttons_pressed: Vec<MouseButton>,
    },
    Scroll {
        delta_y: f32,
        delta_time_ms: f32,
    },
    MouseClick(MouseClick),
}
impl Event {
    pub fn from_map(map: JsMap) -> Self {
        let name: String = map.get(&JsValue::from_str("name")).as_string().unwrap();
        match name.as_str() {
            "mouse_move" => Self::from_mouse_move_map(map),
            "wheel" => Self::from_wheel_map(map),
            _ => panic!("invalid name"),
        }
    }
    pub fn from_wheel_map(map: JsMap) -> Self {
        let delta_y = map.get(&JsValue::from_str("delta_y")).as_f64().unwrap() as f32;
        let delta_time_ms = map
            .get(&JsValue::from_str("delta_time_ms"))
            .as_f64()
            .unwrap() as f32;
        Event::Scroll {
            delta_y,
            delta_time_ms,
        }
    }
    pub fn from_mouse_move_map(map: JsMap) -> Self {
        let buttons_pressed_number: i32 =
            map.get(&JsValue::from_str("buttons")).as_f64().unwrap() as i32;
        let buttons_pressed = match buttons_pressed_number {
            0 => vec![],
            1 => vec![MouseButton::LeftClick],
            2 => vec![MouseButton::RightClick],
            3 => vec![MouseButton::LeftClick, MouseButton::RightClick],
            4 => vec![MouseButton::MiddleClick],
            5 => vec![MouseButton::LeftClick, MouseButton::MiddleClick],
            6 => vec![MouseButton::MiddleClick, MouseButton::RightClick],
            7 => vec![
                MouseButton::LeftClick,
                MouseButton::MiddleClick,
                MouseButton::RightClick,
            ],
            _ => panic!("invalid button number"),
        };
        let delta_x = map.get(&JsValue::from_str("delta_x")).as_f64().unwrap() as f32;
        let delta_y = map.get(&JsValue::from_str("delta_y")).as_f64().unwrap() as f32;
        let delta_time_ms = map
            .get(&JsValue::from_str("delta_time_ms"))
            .as_f64()
            .unwrap() as f32;
        Event::MouseMove {
            delta_x,
            delta_y,
            buttons_pressed,
            delta_time_ms,
        }
    }
}
pub fn log(s: &str) {
    web_sys::console::log(&JsArray::from(&JsValue::from(s)));
}
pub fn log_js_value(s: &JsValue) {
    web_sys::console::log(&JsArray::from(s));
}
struct Game {}
impl Game {
    pub fn new() -> Result<Game, JsValue> {
        Ok(Game {})
    }
}
#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}
#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen]
    pub fn render_frame(&mut self, event_state: JsMap, events: JsArray) {
        let events: Vec<Event> = events.iter().map(|v| Event::from_map(v.into())).collect();

        //    self.engine
        //        .render_frame(to_event_state(&event_state), events)
        //        .ok()
        //        .unwrap();
    }
}
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
pub fn init_game() -> WebGame {
    let r = Game::new();
    if r.is_ok() {
        WebGame { game: Game {} }
    } else {
        log(&format!("{:?}", r.err().unwrap()));
        panic!()
    }
}
