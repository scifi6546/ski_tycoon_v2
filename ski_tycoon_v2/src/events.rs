use js_sys::Map as JsMap;
use log::info;
use nalgebra::{Vector2, Vector3};
use wasm_bindgen::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub enum MouseButton {
    LeftClick,
    MiddleClick,
    RightClick,
}
#[derive(Clone, Debug)]
pub struct MouseClick {
    position: Vector2<f32>,
    button_pressed: MouseButton,
}
#[derive(Clone, Debug)]
pub enum Event {
    MouseMove {
        delta_x: f32,
        delta_y: f32,
        x: f32,
        y: f32,
        delta_time_ms: f32,
        buttons_pressed: Vec<MouseButton>,
    },
    ScreenSizeChange {
        new_size: Vector2<u32>,
    },
    Scroll {
        delta_y: f32,
        delta_time_ms: f32,
    },
    CameraMove {
        direction: Vector3<f32>,
    },
    MouseDown {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    MouseUp {
        x: f32,
        y: f32,
    },
    MouseClick(MouseClick),
}
impl Event {
    pub fn from_map(map: JsMap) -> Option<Self> {
        let name: String = map.get(&JsValue::from_str("name")).as_string().unwrap();
        match name.as_str() {
            "mouse_move" => Some(Self::from_mouse_move_map(map)),
            "mousedown" => Self::from_mousedown(map),
            "mouseup" => Self::from_mouseup(map),
            "wheel" => Some(Self::from_wheel_map(map)),
            "keypress" => Self::from_keypress(map),
            "screen_size_change" => Some(Self::from_screen_size_change(map)),
            _ => panic!("invalid name"),
        }
    }
    fn from_screen_size_change(map: JsMap) -> Self {
        let x = map.get(&"x".into()).as_f64().unwrap() as u32;
        let y = map.get(&"y".into()).as_f64().unwrap() as u32;
        Self::ScreenSizeChange {
            new_size: Vector2::new(x, y),
        }
    }
    fn from_mousedown(map: JsMap) -> Option<Self> {
        let button_i32 = map.get(&JsValue::from_str("buttons")).as_f64().unwrap();
        let button_vec = Self::get_mouse_button(button_i32 as i32);
        if button_vec.is_empty() {
            None
        } else {
            let button = button_vec[0].clone();
            info!("button: {:?}", button);
            let x = map.get(&JsValue::from_str("x")).as_f64().unwrap() as f32;
            let y = map.get(&JsValue::from_str("y")).as_f64().unwrap() as f32;
            Some(Self::MouseDown { button, x, y })
        }
    }
    fn from_mouseup(map: JsMap) -> Option<Self> {
        let button_i32 = map.get(&JsValue::from_str("buttons")).as_f64().unwrap();
        let button_vec = Self::get_mouse_button(button_i32 as i32);
        if button_vec.is_empty() {
            let x = map.get(&JsValue::from_str("x")).as_f64().unwrap() as f32;
            let y = map.get(&JsValue::from_str("y")).as_f64().unwrap() as f32;
            Some(Self::MouseUp { x, y })
        } else {
            let button = button_vec[0].clone();
            info!("button: {:?}", button);
            let x = map.get(&JsValue::from_str("x")).as_f64().unwrap() as f32;
            let y = map.get(&JsValue::from_str("y")).as_f64().unwrap() as f32;
            Some(Self::MouseUp { x, y })
        }
    }
    fn from_keypress(map: JsMap) -> Option<Self> {
        let key: String = map.get(&JsValue::from_str("key")).as_string().unwrap();
        match key.as_str() {
            "w" => Some(Self::CameraMove {
                direction: Vector3::new(0.0, 0.0, -1.0),
            }),
            "a" => Some(Self::CameraMove {
                direction: Vector3::new(-1.0, 0.0, 0.0),
            }),
            "s" => Some(Self::CameraMove {
                direction: Vector3::new(0.0, 0.0, 1.0),
            }),
            "d" => Some(Self::CameraMove {
                direction: Vector3::new(1.0, 0.0, 0.0),
            }),
            _ => None,
        }
    }
    fn from_wheel_map(map: JsMap) -> Self {
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
    fn get_mouse_button(mouse: i32) -> Vec<MouseButton> {
        match mouse {
            0 => vec![],
            1 => vec![MouseButton::RightClick],
            2 => vec![MouseButton::LeftClick],
            3 => vec![MouseButton::LeftClick, MouseButton::RightClick],
            4 => vec![MouseButton::MiddleClick],
            5 => vec![MouseButton::RightClick, MouseButton::MiddleClick],
            6 => vec![MouseButton::MiddleClick, MouseButton::LeftClick],
            7 => vec![
                MouseButton::LeftClick,
                MouseButton::MiddleClick,
                MouseButton::RightClick,
            ],
            _ => panic!("invalid button number"),
        }
    }
    fn from_mouse_move_map(map: JsMap) -> Self {
        let buttons_pressed_number: i32 =
            map.get(&JsValue::from_str("buttons")).as_f64().unwrap() as i32;
        let buttons_pressed = Self::get_mouse_button(buttons_pressed_number);
        let delta_x = map.get(&JsValue::from_str("delta_x")).as_f64().unwrap() as f32;
        let delta_y = map.get(&JsValue::from_str("delta_y")).as_f64().unwrap() as f32;
        let x = map.get(&JsValue::from_str("x")).as_f64().unwrap() as f32;
        let y = map.get(&JsValue::from_str("y")).as_f64().unwrap() as f32;
        let delta_time_ms = map
            .get(&JsValue::from_str("delta_time_ms"))
            .as_f64()
            .unwrap() as f32;
        Event::MouseMove {
            delta_x,
            delta_y,
            x,
            y,
            buttons_pressed,
            delta_time_ms,
        }
    }
}
