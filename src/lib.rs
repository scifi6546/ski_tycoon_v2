mod camera;
mod graphics_engine;
mod graphics_system;
mod gui;
mod model;
mod skiier;
mod utils;
use graphics_engine::{RenderTransform, WebGl};
use js_sys::Array as JsArray;
use log::debug;
use nalgebra::{Vector2, Vector3, Vector4};
mod events;
use camera::Camera;
use events::Event;
use graphics_system::insert_mesh;
use gui::GuiModel;
use legion::*;
use wasm_bindgen::prelude::*;
mod prelude {
    pub use super::camera::Camera;
    pub use super::graphics_engine::{
        ErrorType, Framebuffer, RenderTransform, RuntimeMesh, RuntimeTexture, WebGl,
    };
    pub use super::graphics_engine::{Mesh, RGBATexture as Texture};
    pub use super::graphics_system::RuntimeModel;
    pub use super::gui::{GuiModel, GuiRuntimeModel, GuiTransform};
    #[derive(Clone)]
    pub struct Model {
        pub mesh: super::graphics_engine::Mesh,
        pub texture: super::graphics_engine::RGBATexture,
        pub transform: RenderTransform,
    }
}
struct Game {
    world: World,
    resources: Resources,
}
impl Game {
    pub fn new() -> Result<Game, JsValue> {
        let mut resources = Resources::default();
        let mut world = World::default();
        let mut webgl = WebGl::new()?;
        let transform = RenderTransform::no_transform();
        GuiModel::simple_box(RenderTransform::new_scale(&Vector3::new(0.1, 0.1, 0.1)))
            .insert(&mut world, &mut webgl)?;
        insert_mesh(model::get_cube(transform.clone()), &mut world, &mut webgl)?;
        insert_mesh(
            model::get_terrain_model(Vector2::new(20, 20), transform),
            &mut world,
            &mut webgl,
        )?;

        resources.insert(webgl);
        resources.insert(Camera::new(Vector3::new(0.0, 0.0, 0.0), 20.0, 1.0, 1.0));

        Ok(Game { world, resources })
    }
    pub fn run_frame(&mut self, _events: Vec<Event>) {
        {
            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            debug!("got gl");
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));
        }
        debug!("built scedule");
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_object_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        debug!("executed schedule");
    }
}
#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}
#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen]
    pub fn render_frame(&mut self, events: JsArray) {
        let events: Vec<Event> = events.iter().map(|v| Event::from_map(v.into())).collect();
        debug!("got events");
        self.game.run_frame(events);

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
use log::Level;
#[wasm_bindgen]
pub fn init_game() -> WebGame {
    console_log::init_with_level(Level::Debug).expect("filed to init console_log");
    let r = Game::new();
    if r.is_ok() {
        WebGame {
            game: Game::new().ok().unwrap(),
        }
    } else {
        debug!("create failed");
        panic!()
    }
}
