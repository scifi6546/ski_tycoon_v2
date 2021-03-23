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
#[cfg(not(target_arch = "wasm32"))]
pub use graphics_engine::Window;
use graphics_engine::{
    ErrorType, Framebuffer, InitContext, Mesh, RenderingContext, RuntimeDepthTexture, Transform,
};
use log::{debug, info};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use texture::RGBATexture;
mod events;
use asset_manager::AssetManager;
use bindable::Bindable;
use camera::DeltaCamera;
use events::{Event, MouseButton};
use graph::graph_debug;
//
use graphics_system::{GraphicsSettings, RuntimeModel};
use gui::GuiModel;
use legion::*;
use wasm_bindgen::prelude::*;
pub mod prelude {
    pub use super::asset_manager::AssetManager;
    pub use super::camera::DeltaCamera;
    pub use super::events::{Event, MouseButton};
    pub use super::graph::{
        a_star, dijkstra, FollowPath, GraphLayer, GraphLayerList, GraphWeight, GridNode, LiftLayer,
        Node, NodeFloat, Path,
    };
    pub use super::graphics_engine::{
        ErrorType, Framebuffer, ItemDesc, Mesh, RenderingContext, RuntimeMesh, RuntimeTexture,
        Shader, Transform, Vertex,
    };
    pub type ShaderBind = super::Bindable<Shader>;
    pub use super::graphics_system::{
        insert_terrain, RuntimeDebugMesh, RuntimeModel, RuntimeModelId,
    };
    pub use super::grid::Grid;
    pub use super::gui::{GuiModel, GuiRuntimeModel, GuiTransform};
    pub use super::lift::insert_lift;
    pub use super::model::Model;
    pub use super::skiier::build_skiier;
    pub use super::terrain::Terrain;
    pub use super::texture::RGBATexture as Texture;
    pub use wasm_bindgen::prelude::JsValue;
}
use prelude::ShaderBind;
pub struct Game {
    world: World,
    resources: Resources,
    world_depth_texture: RuntimeDepthTexture,
    world_framebuffer: Framebuffer,
    world_render_surface: RuntimeModel,
}
impl Game {
    pub fn new(screen_size: Vector2<u32>, init_context: InitContext) -> Result<Game, ErrorType> {
        utils::set_panic_hook();
        let mut resources = Resources::default();
        let mut world = World::default();
        let mut webgl = RenderingContext::new(init_context)?;
        let mut shader_bind = Bindable::default();
        let model_manager: AssetManager<RuntimeModel> = AssetManager::default();
        shader_bind.insert("world", webgl.build_world_shader()?);
        shader_bind.bind("world");
        webgl.bind_shader(shader_bind.get_bind()).ok().unwrap();
        webgl
            .send_vec3_uniform(
                &mut shader_bind["world"],
                "sun_direction",
                Vector3::new(1.0, -1.0, 0.0).normalize(),
            )
            .ok()
            .unwrap();
        webgl
            .send_vec4_uniform(
                &mut shader_bind["world"],
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
        let mut world_framebuffer_texture = webgl.build_texture(
            RGBATexture::constant_color(Vector4::new(0, 0, 0, 0), screen_size),
            &shader_bind.get_bind(),
        )?;
        let mut world_depth_texture =
            webgl.build_depth_texture(screen_size, &shader_bind.get_bind())?;
        let fb_mesh = webgl.build_mesh(Mesh::plane(), &shader_bind.get_bind())?;
        let world_framebuffer =
            webgl.build_framebuffer(&mut world_framebuffer_texture, &mut world_depth_texture)?;
        let world_render_surface = RuntimeModel {
            mesh: fb_mesh,
            texture: world_framebuffer_texture,
        };
        webgl.get_error();
        webgl.get_error();
        resources.insert(webgl);
        resources.insert(shader_bind);
        resources.insert(GraphicsSettings { screen_size });
        resources.insert(DeltaCamera::new(
            Vector3::new(0.0, 0.0, 0.0),
            20.0,
            1.0,
            1.0,
        ));
        let (egui_context, egui_adaptor) = gui::init_gui(screen_size);
        resources.insert(egui_context);
        resources.insert(egui_adaptor);
        resources.insert(model_manager);
        resources.insert(lift::BuildLift::default());
        resources.insert(terrain::TerrainLibrary::default());
        // gui::insert_ui(&mut egui_context);
        let g = Game {
            world,
            resources,
            world_depth_texture,
            world_framebuffer,
            world_render_surface,
        };
        Ok(g)
    }
    pub fn run_frame(&mut self, events: Vec<Event>) {
        {
            let camera: &mut DeltaCamera = &mut self.resources.get_mut().unwrap();
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
                    Event::ScreenSizeChange { new_size } => {
                        let shader: &mut ShaderBind = &mut self.resources.get_mut().unwrap();
                        shader.bind("screen");
                        let gl: &mut RenderingContext = &mut self.resources.get_mut().unwrap();
                        gl.change_viewport(new_size).expect("screen updated");
                        let settings: &mut GraphicsSettings =
                            &mut self.resources.get_mut().unwrap();
                        settings.screen_size = *new_size;
                        gl.delete_depth_buffer(&mut self.world_depth_texture)
                            .expect("deleted old texture");
                        gl.delete_texture(&mut self.world_render_surface.texture);
                        gl.delete_mesh(&mut self.world_render_surface.mesh)
                            .expect("failed to delete framebuffer mesh");
                        gl.delete_framebuffer(&mut self.world_framebuffer)
                            .expect("deleted old framebuffer");
                        let mut world_framebuffer_texture = gl
                            .build_texture(
                                RGBATexture::constant_color(Vector4::new(0, 0, 0, 0), *new_size),
                                &shader.get_bind(),
                            )
                            .expect("failed to build new texture");
                        let mut world_depth_texture = gl
                            .build_depth_texture(*new_size, &shader.get_bind())
                            .expect("failed to build depth texture");
                        shader.bind("world");
                        let fb_mesh = gl
                            .build_mesh(Mesh::plane(), &shader.get_bind())
                            .expect("failed to build mesh");

                        shader.bind("screen");
                        self.world_framebuffer = gl
                            .build_framebuffer(
                                &mut world_framebuffer_texture,
                                &mut world_depth_texture,
                            )
                            .expect("failed to build framebuffer");
                        self.world_render_surface = RuntimeModel {
                            mesh: fb_mesh,
                            texture: world_framebuffer_texture,
                        };
                    }
                    Event::CameraMove { direction } => {
                        camera.translate(&(0.1 * direction * camera.get_radius()))
                    }
                    Event::Scroll {
                        delta_y,
                        delta_time_ms,
                    } => {
                        camera.update_radius(0.000001 * delta_y * delta_time_ms);
                        debug!("zoomed");
                    }
                    _ => (),
                }
            }
            camera.apply(&self.world);

            //binding to world framebuffer and rendering to it

            let gl: &mut RenderingContext = &mut self.resources.get_mut().unwrap();
            gl.bind_framebuffer(&self.world_framebuffer);
            gl.clear_screen(Vector4::new(0.2, 0.2, 0.2, 1.0));

            let shader: &mut ShaderBind = &mut self.resources.get_mut().unwrap();
            shader.bind("world");
            gl.bind_shader(shader.get_bind()).ok().unwrap();
        }
        info!("handled sceen resize");
        {
            let library: &terrain::TerrainLibrary = &self.resources.get().unwrap();
            library.draw_gui(
                &mut self.world,
                &mut self.resources.get_mut().unwrap(),
                &mut self.resources.get_mut().unwrap(),
                &mut self.resources.get_mut().unwrap(),
                &self.resources.get().unwrap(),
            );
        }
        //game logic
        skiier::follow_path(&mut self.world);
        {
            let lift: &mut lift::BuildLift = &mut self.resources.get_mut().unwrap();
            lift.build_lift(
                &mut self.world,
                &mut self.resources.get_mut().unwrap(),
                &mut self.resources.get_mut().unwrap(),
                &mut self.resources.get_mut().unwrap(),
                &self.resources.get().unwrap(),
            );
        }
        //rendering susten
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_object_system())
            .build();
        {
            let ctx: &mut egui::CtxRef = &mut self.resources.get_mut().unwrap();
            graph_debug::terrain_debug_window(&self.world, ctx);
            skiier::draw_skiiers(&self.world, ctx);
        }
        schedule.execute(&mut self.world, &mut self.resources);
        {
            let gl: &mut RenderingContext = &mut self.resources.get_mut().unwrap();
            gl.clear_depth();
        }
        let mut schedule = Schedule::builder()
            .add_system(graphics_system::render_debug_system())
            .build();
        schedule.execute(&mut self.world, &mut self.resources);
        info!("rendered debug sustem");
        {
            //binding to world framebuffer and rendering to it

            let gl: &mut RenderingContext = &mut self.resources.get_mut().unwrap();
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
                let settings: &GraphicsSettings = &self.resources.get().unwrap();
                gui::draw_gui(
                    egui_context,
                    &events,
                    gl,
                    shader,
                    egui_adaptor,
                    settings.screen_size,
                )
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
        info!("rendered gui");
    }
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}

#[cfg(target_arch = "wasm32")]
use js_sys::Array as JsArray;
#[cfg(target_arch = "wasm32")]
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
#[wasm_bindgen]
pub struct ScreenResolution {
    pub x: u32,
    pub y: u32,
}
#[wasm_bindgen]
impl ScreenResolution {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn init_game(resolution: js_sys::Map) -> WebGame {
    #[cfg(profile = "dev")]
    console_log::init_with_level(log::Level::Debug).expect("filed to init console_log");
    #[cfg(not(profile = "dev"))]
    console_log::init_with_level(log::Level::Error).expect("filed to init console_log");
    let r = Game::new(
        Vector2::new(
            resolution.get(&("x".into())).as_f64().unwrap() as u32,
            resolution.get(&("y".into())).as_f64().unwrap() as u32,
        ),
        (),
    );
    if r.is_ok() {
        WebGame {
            game: r.ok().unwrap(),
        }
    } else {
        debug!("create failed");
        panic!()
    }
}
