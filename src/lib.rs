mod camera;
mod graphics_engine;
mod graphics_system;
mod gui;
mod model;
mod skiier;
mod utils;

use graphics_engine::{Framebuffer, Mesh, RGBATexture, RenderTransform, WebGl};
use js_sys::Array as JsArray;
use log::debug;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
mod events;
use camera::Camera;
use events::{Event, MouseButton};
use graphics_system::{insert_mesh, RuntimeModel};
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
    world_framebuffer: Framebuffer,
    world_render_surface: RuntimeModel,
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

        let mut fb_texture = webgl.build_texture(RGBATexture::constant_color(
            Vector4::new(0, 0, 0, 0),
            Vector2::new(800, 800),
        ))?;
        let fb_mesh = webgl.build_mesh(Mesh::plane())?;
        let world_framebuffer = webgl.build_framebuffer(&mut fb_texture)?;
        let world_render_surface = RuntimeModel {
            mesh: fb_mesh,
            texture: fb_texture,
        };
        resources.insert(webgl);
        resources.insert(Camera::new(Vector3::new(0.0, 0.0, 0.0), 20.0, 1.0, 1.0));

        Ok(Game {
            world,
            resources,
            world_framebuffer,
            world_render_surface,
        })
    }
    pub fn run_frame(&mut self, events: Vec<Event>) {
        {
            let camera: &mut Camera = &mut self.resources.get_mut().unwrap();
            for e in events {
                match e {
                    Event::MouseMove {
                        delta_x,
                        delta_y,
                        delta_time_ms,
                        buttons_pressed,
                    } => {
                        if buttons_pressed.contains(&MouseButton::RightClick) {
                            camera.rotate_phi(delta_x * 0.001 * delta_time_ms);
                            camera.rotate_theta(delta_y * 0.001 * delta_time_ms);
                        }
                    }
                    Event::CameraZoom {
                        delta_y,
                        delta_time_ms,
                    } => {
                        camera.update_radius(0.001 * delta_y * delta_time_ms);
                        debug!("zoomed");
                    }
                    _ => (),
                }
            }

            //binding to world framebuffer and rendering to it

            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            gl.bind_framebuffer(&self.world_framebuffer);
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));
        }
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_object_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        {
            //binding to world framebuffer and rendering to it

            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            gl.bind_default_framebuffer();
            gl.send_view_matrix(Matrix4::identity());
            gl.send_model_matrix(Matrix4::identity());
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));
            gl.bind_texture(&self.world_render_surface.texture);
            gl.draw_mesh(&self.world_render_surface.mesh);
        }
        let mut gui_schedule = Schedule::builder()
            .add_system(graphics_system::render_gui_system())
            .build();
        gui_schedule.execute(&mut self.world, &mut self.resources);
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
