use js_sys::Map as JsMap;
use nalgebra::{Vector2, Vector3};
use wasm_bindgen::prelude::*;
#[derive(Clone, PartialEq)]
pub enum MouseButton {
    LeftClick,
    MiddleClick,
    RightClick,
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
    CameraZoom {
        delta_y: f32,
        delta_time_ms: f32,
    },
    CameraMove {
        direction: Vector3<f32>,
    },
    MouseClick(MouseClick),
}
impl Event {
    pub fn from_map(map: JsMap) -> Option<Self> {
        let name: String = map.get(&JsValue::from_str("name")).as_string().unwrap();
        match name.as_str() {
            "mouse_move" => Some(Self::from_mouse_move_map(map)),
            "wheel" => Some(Self::from_wheel_map(map)),
            "keypress" => Self::from_keypress(map),
            _ => panic!("invalid name"),
        }
    }
    pub fn from_keypress(map: JsMap) -> Option<Self> {
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
    pub fn from_wheel_map(map: JsMap) -> Self {
        let delta_y = map.get(&JsValue::from_str("delta_y")).as_f64().unwrap() as f32;
        let delta_time_ms = map
            .get(&JsValue::from_str("delta_time_ms"))
            .as_f64()
            .unwrap() as f32;
        Event::CameraZoom {
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
