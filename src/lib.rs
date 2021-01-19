mod asset_manager;
mod bindable;
mod camera;
mod graph;
mod graphics_engine;
mod graphics_system;
mod grid;
mod gui;
mod lift;
mod model;
mod skiier;
mod terrain;
mod texture;
mod utils;
use graphics_engine::{Framebuffer, Mesh, Transform, WebGl};
use js_sys::Array as JsArray;
use log::{debug, info};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use texture::RGBATexture;
mod events;
use bindable::Bindable;
use camera::Camera;
use egui::CtxRef;

use asset_manager::AssetManager;
use events::{Event, MouseButton};
use graphics_system::{insert_terrain, RuntimeModel};
use gui::{EguiRawInputAdaptor, GuiModel};
use legion::*;
use lift::insert_lift;
use terrain::Terrain;
use wasm_bindgen::prelude::*;
pub mod prelude {
    pub use super::asset_manager::AssetManager;
    pub use super::camera::Camera;
    pub use super::graph::{
        dijkstra, find_best_path, FollowPath, GraphLayer, GraphLayerList, GraphWeight, GridNode,
        Node, NodeFloat, Path,
    };
    pub use super::graphics_engine::{
        ErrorType, Framebuffer, RuntimeMesh, RuntimeTexture, Shader, Transform, WebGl,
    };
    pub type ShaderBind = super::Bindable<Shader>;
    pub use super::events::{Event, MouseButton};
    pub use super::graphics_engine::{ItemDesc, Mesh, Vertex};
    pub use super::graphics_system::{RuntimeDebugMesh, RuntimeModel};
    pub use super::grid::Grid;
    pub use super::gui::{GuiModel, GuiRuntimeModel, GuiTransform};
    pub use super::model::Model;
    pub use super::terrain::Terrain;
    pub use super::texture::RGBATexture as Texture;
    pub use wasm_bindgen::prelude::JsValue;
}
use prelude::ShaderBind;
struct Game {
    world: World,
    resources: Resources,
    world_framebuffer: Framebuffer,
    world_render_surface: RuntimeModel,
    //egui_context: CtxRef,
    //egui_adaptor: EguiRawInputAdaptor,
}
impl Game {
    pub fn new() -> Result<Game, JsValue> {
        let mut resources = Resources::default();
        let mut world = World::default();
        let mut webgl = WebGl::new()?;
        let mut shader_bind = Bindable::default();
        let mut model_manager = AssetManager::default();
        shader_bind.insert("world", webgl.build_world_shader()?);
        shader_bind.bind("world");
        webgl.bind_shader(shader_bind.get_bind()).ok().unwrap();
        webgl
            .send_vec3_uniform(
                &shader_bind["world"],
                "sun_direction",
                Vector3::new(1.0, -1.0, 0.0).normalize(),
            )
            .ok()
            .unwrap();
        webgl
            .send_vec4_uniform(
                &shader_bind["world"],
                "sun_color",
                Vector4::new(1.0, 1.0, 1.0, 1.0),
            )
            .ok()
            .unwrap();
        webgl.get_error();
        shader_bind.insert("screen", webgl.build_screen_shader()?);
        shader_bind.insert("gui", webgl.build_gui_shader()?);
        shader_bind.bind("world");
        webgl.bind_shader(shader_bind.get_bind()).ok().unwrap();
        let mut box_transform = Transform::default();
        box_transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
        box_transform.translate(Vector3::new(-0.5, -0.5, 0.0));

        GuiModel::simple_box(box_transform).insert(
            &mut world,
            &mut webgl,
            &shader_bind.get_bind(),
        )?;
        webgl.get_error();
        insert_terrain(
            Terrain::new_cone(Vector2::new(20, 20), Vector2::new(5.0, 5.0), 5.0, -1.0),
            &mut world,
            &mut webgl,
            &mut model_manager,
            &shader_bind.get_bind(),
        )?;
        insert_lift(
            &mut world,
            &mut webgl,
            &mut model_manager,
            &shader_bind,
            Vector2::new(0, 0),
        )?;
        webgl.get_error();
        let mut fb_texture = webgl.build_texture(
            RGBATexture::constant_color(Vector4::new(0, 0, 0, 0), Vector2::new(800, 800)),
            &shader_bind.get_bind(),
        )?;
        let mut fb_depth =
            webgl.build_depth_texture(Vector2::new(800, 800), &shader_bind.get_bind())?;
        let fb_mesh = webgl.build_mesh(Mesh::plane(), &shader_bind.get_bind())?;
        let world_framebuffer = webgl.build_framebuffer(&mut fb_texture, &mut fb_depth)?;
        let world_render_surface = RuntimeModel {
            mesh: fb_mesh,
            texture: fb_texture,
        };
        webgl.get_error();
        for i in 0..10 {
            skiier::build_skiier(
                &mut world,
                &mut webgl,
                &shader_bind,
                Vector2::new(i, 0),
                Vector2::new(10, i),
            )?;
        }
        webgl.get_error();
        resources.insert(webgl);
        resources.insert(shader_bind);
        resources.insert(Camera::new(Vector3::new(0.0, 0.0, 0.0), 20.0, 1.0, 1.0));
        let (egui_context, egui_adaptor) = gui::init_gui();
        resources.insert(egui_context);
        resources.insert(egui_adaptor);
        resources.insert(model_manager);
        // gui::insert_ui(&mut egui_context);
        info!("context created");
        info!("inserted ui");
        let g = Game {
            world,
            resources,
            world_framebuffer,
            world_render_surface,
            //egui_adaptor,
            //egui_context,
        };
        info!("built game successfully");
        Ok(g)
    }
    pub fn run_frame(&mut self, events: Vec<Event>) {
        info!("running frame?");
        {
            let context = &mut self.resources.get_mut().unwrap();
            gui::insert_ui(context);
        }
        {
            let camera: &mut Camera = &mut self.resources.get_mut().unwrap();
            for e in events.iter() {
                match e {
                    Event::MouseMove {
                        delta_x,
                        delta_y,
                        delta_time_ms,
                        buttons_pressed,
                        ..
                    } => {
                        if buttons_pressed.contains(&MouseButton::RightClick) {
                            camera.rotate_phi(delta_x * 0.001 * delta_time_ms);
                            camera.rotate_theta(delta_y * 0.001 * delta_time_ms);
                        }
                    }
                    Event::CameraMove { direction } => camera.translate(&(0.1 * direction)),
                    Event::Scroll {
                        delta_y,
                        delta_time_ms,
                    } => {
                        camera.update_radius(0.0000001 * delta_y * delta_time_ms);
                        debug!("zoomed");
                    }
                    _ => (),
                }
            }

            //binding to world framebuffer and rendering to it

            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            gl.bind_framebuffer(&self.world_framebuffer);
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));

            let shader: &mut ShaderBind = &mut self.resources.get_mut().unwrap();
            shader.bind("world");
            gl.bind_shader(shader.get_bind()).ok().unwrap();
        }
        //game logic
        let mut schedule = Schedule::builder()
            .add_system(skiier::follow_path_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_object_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        {
            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            gl.clear_depth();
        }
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_debug_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        {
            //binding to world framebuffer and rendering to it

            let gl: &mut WebGl = &mut self.resources.get_mut().unwrap();
            let shader: &mut ShaderBind = &mut self.resources.get_mut().unwrap();
            shader.bind("screen");
            gl.bind_default_framebuffer();
            //getting screen shader
            gl.bind_shader(shader.get_bind()).ok().unwrap();
            gl.send_view_matrix(Matrix4::identity(), shader.get_bind());
            gl.send_model_matrix(Matrix4::identity(), shader.get_bind());
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));
            gl.bind_texture(&self.world_render_surface.texture, shader.get_bind());
            gl.draw_mesh(&self.world_render_surface.mesh);

            //binding and drawing gui shader
            shader.bind("gui");
            gl.bind_shader(shader.get_bind()).ok().unwrap();
            {
                let egui_context = &mut self.resources.get_mut().unwrap();
                let egui_adaptor = &mut self.resources.get_mut().unwrap();
                gui::draw_gui(egui_context, &events, gl, shader, egui_adaptor)
                    .expect("successfully drew");
            }
            shader.bind("screen");
            //getting screen shader
            gl.bind_shader(shader.get_bind()).ok().unwrap();
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
        let events: Vec<Event> = events
            .iter()
            .map(|v| Event::from_map(v.into()))
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect();
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
    console_log::init_with_level(Level::Info).expect("filed to init console_log");
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
