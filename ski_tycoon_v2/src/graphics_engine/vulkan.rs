// Vulkan frontent rendering engine
mod shader;
use super::super::prelude::Texture;
use super::Mesh;
use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};
use std::cmp::min;

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use std::ffi::{CStr, CString};
use winit::window::Window;
#[derive(Clone)]
pub struct RuntimeMesh {}
#[derive(Clone)]
pub struct RuntimeTexture {}
pub type ErrorType = ash::vk::Result;
pub struct Framebuffer {}

pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
pub struct RenderingContext {
    entry: Entry,
    instance: Instance,
    logical_device: Device,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    present_queue: vk::Queue,
}
pub struct Shader {}
pub type InitContext = Window;
impl RenderingContext {
    unsafe fn is_device_sutible(
        physical_device: &vk::PhysicalDevice,
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        surface_loader: &Surface,
    ) -> Option<(vk::PhysicalDevice, usize)> {
        instance
            .get_physical_device_queue_family_properties(*physical_device)
            .iter()
            .enumerate()
            .filter_map(|(index, ref info)| {
                let max_image_count = surface_loader
                    .get_physical_device_surface_capabilities(*physical_device, *surface)
                    .expect("failed get capabilities")
                    .max_image_count;

                let is_sutible = info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && surface_loader
                        .get_physical_device_surface_support(
                            *physical_device,
                            index as u32,
                            *surface,
                        )
                        .unwrap()
                    && max_image_count > 0;

                if is_sutible {
                    Some((*physical_device, index))
                } else {
                    None
                }
            })
            .next()
    }
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
        let required_extensions = ash_window::enumerate_required_extensions(window).unwrap();
        let required_extensions_raw: Vec<*const i8> =
            required_extensions.iter().map(|ext| ext.as_ptr()).collect();
        for e in required_extensions.iter() {
            println!(
                "requires extension: {}",
                e.to_str().expect("extension invalid utf")
            );
        }
        let layers: Vec<*const i8> = Self::LAYER_NAMES
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect();
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&required_extensions_raw);
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Error in creating instance")
        };

        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, window, None)
                .expect("could not create surface")
        };
        let surface_loader = Surface::new(&entry, &instance);
        unsafe {
            let (physical_device, queue_family_index) = instance
                .enumerate_physical_devices()
                .expect("Could not find physical device")
                .iter()
                .filter_map(|pdevice| {
                    Self::is_device_sutible(pdevice, &instance, &surface, &surface_loader)
                })
                .next()
                .expect("Could Not find a sutioble device");
            let priorities = [1.0];
            let queue_info = [vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index as u32)
                .queue_priorities(&priorities)
                .build()];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
            let device_extension_names_raw = [Swapchain::name().as_ptr()];
            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);
            let logical_device = instance
                .create_device(physical_device, &device_create_info, None)
                .expect("failed to create logical device");
            let present_queue = logical_device.get_device_queue(queue_family_index as u32, 0);
            let surface_format = surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()[0];
            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)?;
            let desired_image_count = min(
                surface_capabilities.min_image_count + 1,
                surface_capabilities.max_image_count,
            );
            println!("max image count: {}", surface_capabilities.max_image_count);
            let surface_resolution = match surface_capabilities.current_extent.width {
                std::u32::MAX => vk::Extent2D {
                    width: window.inner_size().width,
                    height: window.inner_size().height,
                },
                _ => surface_capabilities.current_extent,
            };
            let pre_transform = if surface_capabilities
                .supported_transforms
                .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
            {
                vk::SurfaceTransformFlagsKHR::IDENTITY
            } else {
                surface_capabilities.current_transform
            };
            let present_modes = surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap();
            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
                .unwrap_or(vk::PresentModeKHR::FIFO);
            let swapchain_loader = Swapchain::new(&instance, &logical_device);
            let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface)
                .min_image_count(desired_image_count)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(pre_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1);
            let swapchain = swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap();
            println!("desired image count: {}", desired_image_count);
            Ok(Self {
                entry,
                instance,
                logical_device,
                surface,
                swapchain,
                present_queue,
            })
        }
    }
    pub fn change_viewport(&self, screen_size: &Vector2<u32>) -> Result<(), ErrorType> {
        todo!()
    }
    pub fn build_world_shader(&mut self) -> Result<Shader, ErrorType> {
        let world_shader = shader::get_world();
        let frag_create_info =
            vk::ShaderModuleCreateInfo::builder().code(&world_shader.fragment_shader_data);
        let vert_create_info =
            vk::ShaderModuleCreateInfo::builder().code(&world_shader.vertex_shader_data);

        unsafe {
            self.logical_device
                .create_shader_module(&frag_create_info, None)?;
            self.logical_device
                .create_shader_module(&vert_create_info, None)?;
        }
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
