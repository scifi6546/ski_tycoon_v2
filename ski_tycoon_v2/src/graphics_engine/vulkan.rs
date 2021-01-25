// Vulkan frontent rendering engine
use super::super::prelude::Texture;
use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};

pub use super::shader::{Shader, ShaderText};
use super::Mesh;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use std::ffi::{CStr, CString};
use winit::window::Window;
#[derive(Clone)]
pub struct RuntimeMesh {}
#[derive(Clone)]
pub struct RuntimeTexture {}
#[derive(Debug)]
pub struct ErrorType {}
pub struct Framebuffer {}

pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
pub struct RenderingContext {
    entry: Entry,
    instance: Instance,
}
pub type InitContext = Window;
impl RenderingContext {
    const APP_NAME: &'static str = "Ski Tycoon";
    const LAYER_NAMES: &'static [&'static [u8]] = &[b"VK_LAYER_KHRONOS_validation\0"];
    pub fn new(window: &Window) -> Result<Self, ErrorType> {
        let entry = Entry::new().unwrap();
        let app_name = CString::new(Self::APP_NAME).expect("created app name");
        let application_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk::make_version(1, 0, 0));
        let required_extensions: Vec<*const i8> = ash_window::enumerate_required_extensions(window)
            .unwrap()
            .iter()
            .map(|ext| ext.as_ptr())
            .collect();
        let layers: Vec<*const i8> = Self::LAYER_NAMES
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect();
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&required_extensions);
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Error in creating instance")
        };
        Ok(Self { entry, instance })
    }
    pub fn change_viewport(&self, screen_size: &Vector2<u32>) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn build_world_shader(&mut self) -> Result<Shader, ErrorType> {
        todo!()
    }
    /// Builds shader used for screenspace
    pub fn build_screen_shader(&mut self) -> Result<Shader, ErrorType> {
        todo!()
    }
    pub fn build_gui_shader(&mut self) -> Result<Shader, ErrorType> {
        todo!()
    }
    pub fn bind_shader(&mut self, shader: &Shader) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn build_mesh(&mut self, mesh: Mesh, shader: &Shader) -> Result<RuntimeMesh, ErrorType> {
        todo!()
    }
    pub fn delete_mesh(&mut self, mesh: &mut RuntimeMesh) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn send_vec3_uniform(
        &self,
        shader: &Shader,
        uniform_name: &str,
        data: Vector3<f32>,
    ) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn send_vec4_uniform(
        &self,
        shader: &Shader,
        uniform_name: &str,
        data: Vector4<f32>,
    ) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn build_depth_texture(
        &mut self,
        dimensions: Vector2<u32>,
        shader: &Shader,
    ) -> Result<RuntimeDepthTexture, ErrorType> {
        todo!()
    }
    pub fn delete_depth_buffer(
        &mut self,
        texture: &mut RuntimeDepthTexture,
    ) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn build_texture(
        &mut self,
        texture: Texture,
        shader: &Shader,
    ) -> Result<RuntimeTexture, ErrorType> {
        todo!()
    }
    pub fn delete_texture(&mut self, texture: &mut RuntimeTexture) {
        todo!()
    }
    pub fn build_framebuffer(
        &mut self,
        texture_attachment: &mut RuntimeTexture,
        depth_attachment: &mut RuntimeDepthTexture,
    ) -> Result<Framebuffer, ErrorType> {
        todo!()
    }
    pub fn delete_framebuffer(&mut self, framebuffer: &mut Framebuffer) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn bind_default_framebuffer(&mut self) {
        todo!()
    }
    pub fn clear_screen(&mut self, color: Vector4<f32>) {
        todo!()
    }
    pub fn clear_depth(&mut self) {
        todo!()
    }
    pub fn bind_texture(&mut self, texture: &RuntimeTexture, shader: &Shader) {
        todo!()
    }
    pub fn bind_framebuffer(&mut self, framebuffer: &Framebuffer) {
        todo!()
    }
    pub fn draw_mesh(&mut self, mesh: &RuntimeMesh) {
        todo!()
    }
    pub fn draw_lines(&mut self, mesh: &RuntimeMesh) {
        todo!()
    }
    pub fn send_model_matrix(&mut self, matrix: Matrix4<f32>, shader: &Shader) {
        todo!()
    }
    pub fn send_view_matrix(&mut self, matrix: Matrix4<f32>, shader: &Shader) {
        todo!()
    }
    pub fn get_error(&self) {
        todo!()
    }
}
