// Vulkan frontent rendering engine
mod shader;
use super::super::prelude::Texture;
use super::Mesh;

use std::cmp::min;
use std::collections::HashMap;

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use std::ffi::{CStr, CString};
use winit::window::Window;
#[derive(Clone)]
pub struct RuntimeMesh {}
#[derive(Clone)]
pub struct RuntimeTexture {}
pub type ErrorType = ();
pub struct Framebuffer {}

pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
pub struct RenderingContext {}
pub struct Shader {
    key: String,
}

pub type InitContext = Window;
impl RenderingContext {
    pub fn new(window: &Window) -> Result<Self, ErrorType> {
        todo!()
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
//should be fine
unsafe impl Send for RenderingContext {}
