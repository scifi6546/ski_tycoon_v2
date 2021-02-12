// gfx frontent rendering engine
mod shader;
use super::super::prelude::Texture;
use super::Mesh;
use gfx_hal::{
    adapter::PhysicalDevice,
    buffer, command,
    command::CommandBuffer,
    device::Device,
    format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
    memory, pass,
    pass::Subpass,
    pool,
    pool::CommandPool,
    pso,
    pso::{DescriptorPool, PipelineStage, ShaderStageFlags, VertexInputRate},
    queue::{CommandQueue, QueueFamily, QueueGroup},
    window,
    window::{PresentationSurface, Surface},
    Instance,
};

use std::collections::HashMap;
use std::{cmp::min, io::Cursor, iter, mem, mem::ManuallyDrop, ptr};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use std::ffi::{CStr, CString};

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as back;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
)))]
extern crate gfx_backend_empty as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

#[derive(Clone)]
pub struct RuntimeMesh {}
#[derive(Clone)]
pub struct RuntimeTexture {}
pub type ErrorType = ();
pub struct Framebuffer {}

pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
#[allow(dead_code)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}
const QUAD: [Vertex; 6] = [
    Vertex {
        pos: [-1.0, 1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 0.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, 0.0],
        uv: [0.0, 0.0],
    },
];
pub struct Window<B: gfx_hal::Backend> {
    pub instance: B::Instance,
    pub surface: B::Surface,
    pub adapter: gfx_hal::adapter::Adapter<B>,
    pub window_dimensions: window::Extent2D,
}
pub struct GfxRenderingContext<B: gfx_hal::Backend> {
    desc_pool: ManuallyDrop<B::DescriptorPool>,
    surface: ManuallyDrop<B::Surface>,
    format: gfx_hal::format::Format,
    dimensions: window::Extent2D,
    viewport: pso::Viewport,
    render_pass: ManuallyDrop<B::RenderPass>,
    framebuffer: ManuallyDrop<B::Framebuffer>,
    pipeline: ManuallyDrop<B::GraphicsPipeline>,
    pipeline_layout: ManuallyDrop<B::PipelineLayout>,
    desc_set: Option<B::DescriptorSet>,
    set_layout: ManuallyDrop<B::DescriptorSetLayout>,
    submission_complete_semaphores: Vec<B::Semaphore>,
    submission_complete_fences: Vec<B::Fence>,
    cmd_pools: Vec<B::CommandPool>,
    cmd_buffers: Vec<B::CommandBuffer>,
    vertex_buffer: ManuallyDrop<B::Buffer>,
    image_upload_buffer: ManuallyDrop<B::Buffer>,
    image_logo: ManuallyDrop<B::Image>,
    image_srv: ManuallyDrop<B::ImageView>,
    buffer_memory: ManuallyDrop<B::Memory>,
    image_memory: ManuallyDrop<B::Memory>,
    image_upload_memory: ManuallyDrop<B::Memory>,
    sampler: ManuallyDrop<B::Sampler>,
    frames_in_flight: usize,
    frame: u64,
    //should be dropped in decleration order
    device: B::Device,
    adapter: gfx_hal::adapter::Adapter<B>,
    queue_group: QueueGroup<B>,
    instance: B::Instance,
}
pub struct Shader {}
pub type RenderingContext = GfxRenderingContext<back::Backend>;
pub type InitContext = Window<back::Backend>;
const DIMS: window::Extent2D = window::Extent2D {
    width: 1024,
    height: 768,
};
impl<B: gfx_hal::Backend> GfxRenderingContext<B> {
    pub fn new(mut window: Window<B>) -> Result<Self, ErrorType> {
        let window_dimensions = DIMS;
        let memory_types = window
            .adapter
            .physical_device
            .memory_properties()
            .memory_types;
        let limits = window.adapter.physical_device.limits();
        let family = window
            .adapter
            .queue_families
            .iter()
            .find(|family| {
                window.surface.supports_queue_family(family)
                    && family.queue_type().supports_graphics()
            })
            .expect("No queue family supports presentation");
        let mut gpu = unsafe {
            window
                .adapter
                .physical_device
                .open(&[(family, &[1.0])], gfx_hal::Features::empty())
                .unwrap()
        };
        let mut queue_group = gpu.queue_groups.pop().unwrap();
        let device = gpu.device;
        let mut command_pool = unsafe {
            device.create_command_pool(queue_group.family, pool::CommandPoolCreateFlags::empty())
        }
        .expect("failed to create_command pool");
        let set_layout = ManuallyDrop::new(
            unsafe {
                device.create_descriptor_set_layout(
                    vec![
                        pso::DescriptorSetLayoutBinding {
                            binding: 0,
                            ty: pso::DescriptorType::Image {
                                ty: pso::ImageDescriptorType::Sampled {
                                    with_sampler: false,
                                },
                            },
                            count: 1,
                            stage_flags: ShaderStageFlags::FRAGMENT,
                            immutable_samplers: false,
                        },
                        pso::DescriptorSetLayoutBinding {
                            binding: 1,
                            ty: pso::DescriptorType::Sampler,
                            count: 1,
                            stage_flags: ShaderStageFlags::FRAGMENT,
                            immutable_samplers: false,
                        },
                    ]
                    .into_iter(),
                    iter::empty(),
                )
            }
            .expect("failed to create descriptor set layouyt"),
        );
        let mut desc_pool = ManuallyDrop::new(
            unsafe {
                device.create_descriptor_pool(
                    1,
                    vec![
                        pso::DescriptorRangeDesc {
                            ty: pso::DescriptorType::Image {
                                ty: pso::ImageDescriptorType::Sampled {
                                    with_sampler: false,
                                },
                            },
                            count: 1,
                        },
                        pso::DescriptorRangeDesc {
                            ty: pso::DescriptorType::Sampler,
                            count: 1,
                        },
                    ]
                    .into_iter(),
                    pso::DescriptorPoolCreateFlags::empty(),
                )
            }
            .expect("failed to create desc pool"),
        );
        let mut desc_set = unsafe { desc_pool.allocate_one(&set_layout) }.unwrap();
        println!("Memory Types: {:?}", memory_types);
        let non_coherent_alignment = limits.non_coherent_atom_size as u64;
        let buffer_stride = mem::size_of::<Vertex>() as u64;
        let buffer_len = QUAD.len() as u64 * buffer_stride;
        let padded_buffer_len = ((buffer_len + non_coherent_alignment - 1)
            / non_coherent_alignment)
            * non_coherent_alignment;
        let mut vertex_buffer = ManuallyDrop::new(
            unsafe { device.create_buffer(padded_buffer_len, buffer::Usage::VERTEX) }.unwrap(),
        );

        let buffer_req = unsafe { device.get_buffer_requirements(&vertex_buffer) };
        let upload_type = memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                // type_mask is a bit field where each bit represents a memory type. If the bit is set
                // to 1 it means we can use that type for our buffer. So this code finds the first
                // memory type that has a `1` (or, is allowed), and is visible to the CPU.
                buffer_req.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();
        let buffer_memory = unsafe {
            let mut memory = device
                .allocate_memory(upload_type, buffer_req.size)
                .unwrap();
            device
                .bind_buffer_memory(&memory, 0, &mut vertex_buffer)
                .unwrap();
            let mapping = device
                .map_memory(&mut memory, memory::Segment::ALL)
                .unwrap();
            ptr::copy_nonoverlapping(QUAD.as_ptr() as *const u8, mapping, buffer_len as usize);
            device
                .flush_mapped_memory_ranges(iter::once((&memory, memory::Segment::ALL)))
                .unwrap();
            device.unmap_memory(&mut memory);
            ManuallyDrop::new(memory)
        };
        let img_data = include_bytes!("./gfx/data/logo.png");
        let img = image::load(Cursor::new(&img_data[..]), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let (width, height) = img.dimensions();
        let kind = gfx_hal::image::Kind::D2(
            width as gfx_hal::image::Size,
            height as gfx_hal::image::Size,
            1,
            1,
        );
        let row_alignment_mask = limits.optimal_buffer_copy_pitch_alignment as u32 - 1;
        let image_stride = 4usize;
        let row_pitch = (width * image_stride as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = (height * row_pitch) as u64;
        let padded_upload_size = ((upload_size + non_coherent_alignment - 1)
            / non_coherent_alignment)
            * non_coherent_alignment;

        let mut image_upload_buffer = ManuallyDrop::new(
            unsafe { device.create_buffer(padded_upload_size, buffer::Usage::TRANSFER_SRC) }
                .expect("failed to create image upload buffer"),
        );
        let image_mem_reqs = unsafe { device.get_buffer_requirements(&image_upload_buffer) };
        let image_upload_memory = unsafe {
            let mut memory = device
                .allocate_memory(upload_type, image_mem_reqs.size)
                .unwrap();
            device
                .bind_buffer_memory(&memory, 0, &mut image_upload_buffer)
                .expect("failed to bind image memory");
            let mapping = device
                .map_memory(&mut memory, memory::Segment::ALL)
                .unwrap();
            for y in 0..height as usize {
                let row = &(*img)
                    [width as usize * y * image_stride..width as usize * (y + 1) * image_stride];
                ptr::copy_nonoverlapping(
                    row.as_ptr(),
                    mapping.offset(y as isize * row_pitch as isize),
                    width as usize * image_stride,
                );
            }
            device
                .flush_mapped_memory_ranges(iter::once((&memory, gfx_hal::memory::Segment::ALL)))
                .unwrap();
            device.unmap_memory(&mut memory);
            ManuallyDrop::new(memory)
        };
        let mut image_logo = ManuallyDrop::new(
            unsafe {
                device.create_image(
                    kind,
                    1,
                    ColorFormat::SELF,
                    gfx_hal::image::Tiling::Optimal,
                    gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
                    gfx_hal::image::ViewCapabilities::empty(),
                )
            }
            .unwrap(),
        );
        let image_req = unsafe { device.get_image_requirements(&image_logo) };
        let device_type = memory_types
            .iter()
            .enumerate()
            .position(|(id, memory_type)| {
                image_req.type_mask & (1 << id) != 0
                    && memory_type
                        .properties
                        .contains(gfx_hal::memory::Properties::DEVICE_LOCAL)
            })
            .unwrap()
            .into();
        println!("device type: {:?}", device_type);
        let image_memory = ManuallyDrop::new(
            unsafe { device.allocate_memory(device_type, image_req.size) }.unwrap(),
        );
        unsafe { device.bind_image_memory(&image_memory, 0, &mut image_logo) }.unwrap();
        let image_srv = ManuallyDrop::new(
            unsafe {
                device.create_image_view(
                    &image_logo,
                    gfx_hal::image::ViewKind::D2,
                    ColorFormat::SELF,
                    Swizzle::NO,
                    gfx_hal::image::SubresourceRange {
                        aspects: gfx_hal::format::Aspects::COLOR,
                        ..Default::default()
                    },
                )
            }
            .unwrap(),
        );
        let sampler = ManuallyDrop::new(
            unsafe {
                device.create_sampler(&gfx_hal::image::SamplerDesc::new(
                    gfx_hal::image::Filter::Linear,
                    gfx_hal::image::WrapMode::Clamp,
                ))
            }
            .unwrap(),
        );
        unsafe {
            device.write_descriptor_set(pso::DescriptorSetWrite {
                set: &mut desc_set,
                binding: 0,
                array_offset: 0,
                descriptors: vec![
                    pso::Descriptor::Image(
                        &*image_srv,
                        gfx_hal::image::Layout::ShaderReadOnlyOptimal,
                    ),
                    pso::Descriptor::Sampler(&*sampler),
                ]
                .into_iter(),
            });
        }
        let mut copy_fence = device.create_fence(false).expect("failed to create fence");
        //copying image
        unsafe {
            let mut cmd_buffer = command_pool.allocate_one(command::Level::Primary);
            cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
            let image_barrier = gfx_hal::memory::Barrier::Image {
                states: (
                    gfx_hal::image::Access::empty(),
                    gfx_hal::image::Layout::Undefined,
                )
                    ..(
                        gfx_hal::image::Access::TRANSFER_WRITE,
                        gfx_hal::image::Layout::TransferDstOptimal,
                    ),
                target: &*image_logo,
                families: None,
                range: gfx_hal::image::SubresourceRange {
                    aspects: gfx_hal::format::Aspects::COLOR,
                    ..Default::default()
                },
            };
            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                gfx_hal::memory::Dependencies::empty(),
                iter::once(image_barrier),
            );
            cmd_buffer.copy_buffer_to_image(
                &image_upload_buffer,
                &image_logo,
                gfx_hal::image::Layout::TransferDstOptimal,
                iter::once(command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: row_pitch / (image_stride as u32),
                    buffer_height: height,
                    image_layers: gfx_hal::image::SubresourceLayers {
                        aspects: gfx_hal::format::Aspects::COLOR,
                        level: 0,
                        layers: 0..1,
                    },
                    image_offset: gfx_hal::image::Offset { x: 0, y: 0, z: 0 },
                    image_extent: gfx_hal::image::Extent {
                        width,
                        height,
                        depth: 1,
                    },
                }),
            );
            let image_barrier = gfx_hal::memory::Barrier::Image {
                states: (
                    gfx_hal::image::Access::TRANSFER_WRITE,
                    gfx_hal::image::Layout::TransferDstOptimal,
                )
                    ..(
                        gfx_hal::image::Access::SHADER_READ,
                        gfx_hal::image::Layout::ShaderReadOnlyOptimal,
                    ),
                target: &*image_logo,
                families: None,
                range: gfx_hal::image::SubresourceRange {
                    aspects: gfx_hal::format::Aspects::COLOR,
                    ..Default::default()
                },
            };
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                gfx_hal::memory::Dependencies::empty(),
                iter::once(image_barrier),
            );
            cmd_buffer.finish();
            queue_group.queues[0].submit(
                iter::once(&cmd_buffer),
                iter::empty(),
                iter::empty(),
                Some(&mut copy_fence),
            );
            device
                .wait_for_fence(&copy_fence, !0)
                .expect("failed to wait for fence");
            device.destroy_fence(copy_fence);
        }
        let caps = window.surface.capabilities(&window.adapter.physical_device);
        let formats = window
            .surface
            .supported_formats(&window.adapter.physical_device);
        println!("formats: {:?}", formats);
        //checking supported formats
        let format = formats.map_or(gfx_hal::format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });
        let swap_config = window::SwapchainConfig::from_caps(&caps, format, window_dimensions);
        let fat = swap_config.framebuffer_attachment();
        let extent = swap_config.extent;
        unsafe {
            window
                .surface
                .configure_swapchain(&device, swap_config)
                .expect("Can not config swapchain")
        };
        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(format),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: gfx_hal::image::Layout::Undefined..gfx_hal::image::Layout::Present,
            };
            let subpass = pass::SubpassDesc {
                colors: &[(0, gfx_hal::image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };
            ManuallyDrop::new(
                unsafe {
                    device.create_render_pass(
                        iter::once(attachment),
                        iter::once(subpass),
                        iter::empty(),
                    )
                }
                .expect("failed to create render pass"),
            )
        };
        let framebuffer = ManuallyDrop::new(
            unsafe {
                device.create_framebuffer(
                    &render_pass,
                    iter::once(fat),
                    gfx_hal::image::Extent {
                        width: window_dimensions.width,
                        height: window_dimensions.height,
                        depth: 1,
                    },
                )
            }
            .expect("failed to create framebuffer"),
        );
        //maximum number of frames that can be computed at the same time
        let frames_in_flight = 3;
        let mut submission_complete_semaphores = Vec::with_capacity(frames_in_flight);
        let mut submission_complete_fences = Vec::with_capacity(frames_in_flight);

        // Note: We don't really need a different command pool per frame in such a simple demo like this,
        // but in a more 'real' application, it's generally seen as optimal to have one command pool per
        // thread per frame. There is a flag that lets a command pool reset individual command buffers
        // which are created from it, but by default the whole pool (and therefore all buffers in it)
        // must be reset at once. Furthermore, it is often the case that resetting a whole pool is actually
        // faster and more efficient for the hardware than resetting individual command buffers, so it's
        // usually best to just make a command pool for each set of buffers which need to be reset at the
        // same time (each frame). In our case, each pool will only have one command buffer created from it,
        // though.
        let mut cmd_pools = Vec::with_capacity(frames_in_flight);
        let mut cmd_buffers = Vec::with_capacity(frames_in_flight);
        cmd_pools.push(command_pool);
        for _ in 1..frames_in_flight {
            unsafe {
                cmd_pools.push(
                    device
                        .create_command_pool(
                            queue_group.family,
                            pool::CommandPoolCreateFlags::empty(),
                        )
                        .expect("failed to create command pool"),
                );
            }
        }
        for i in 0..frames_in_flight {
            submission_complete_semaphores.push(
                device
                    .create_semaphore()
                    .expect("failed to create semaphore"),
            );
            submission_complete_fences
                .push(device.create_fence(true).expect("failed to create fence"));
            cmd_buffers.push(unsafe { cmd_pools[i].allocate_one(command::Level::Primary) });
        }
        let pipeline_layout = ManuallyDrop::new(
            unsafe { device.create_pipeline_layout(iter::once(&*set_layout), iter::empty()) }
                .expect("failed to create pipeline layout"),
        );
        let pipeline = {
            let vs_module = {
                let spirv = gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
                    "./gfx/data/shader.vert.spv"
                )))
                .unwrap();
                unsafe { device.create_shader_module(&spirv).unwrap() }
            };
            let fs_module = {
                let spirv = gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
                    "./gfx/data/shader.frag.spv"
                )))
                .unwrap();
                unsafe { device.create_shader_module(&spirv).unwrap() }
            };
            let (vs_entry, fs_entry) = {
                (
                    pso::EntryPoint {
                        entry: "main",
                        module: &vs_module,
                        specialization: gfx_hal::spec_const_list![0.8f32],
                    },
                    pso::EntryPoint {
                        entry: "main",
                        module: &fs_module,
                        specialization: pso::Specialization::default(),
                    },
                )
            };
            let subpass = Subpass {
                index: 0,
                main_pass: &*render_pass,
            };
            let vertex_buffers = vec![pso::VertexBufferDesc {
                binding: 0,
                stride: mem::size_of::<Vertex>() as u32,
                rate: VertexInputRate::Vertex,
            }];
            let attributes = vec![
                pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: gfx_hal::format::Format::Rg32Sfloat,
                        offset: 0,
                    },
                },
                pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: gfx_hal::format::Format::Rg32Sfloat,
                        offset: (std::mem::size_of::<f32>() * 3) as u32,
                    },
                },
            ];
            let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                pso::PrimitiveAssemblerDesc::Vertex {
                    buffers: &vertex_buffers,
                    attributes: &attributes,
                    input_assembler: pso::InputAssemblerDesc {
                        primitive: pso::Primitive::TriangleList,
                        with_adjacency: false,
                        restart_index: None,
                    },
                    vertex: vs_entry,
                    geometry: None,
                    tessellation: None,
                },
                pso::Rasterizer::FILL,
                Some(fs_entry),
                &*pipeline_layout,
                subpass,
            );
            pipeline_desc.blender.targets.push(pso::ColorBlendDesc {
                mask: pso::ColorMask::ALL,
                blend: Some(pso::BlendState::ALPHA),
            });
            let pipeline = unsafe {
                device
                    .create_graphics_pipeline(&pipeline_desc, None)
                    .unwrap()
            };
            unsafe {
                device.destroy_shader_module(vs_module);
                device.destroy_shader_module(fs_module);
            }
            pipeline
        };
        let viewport = pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: extent.width as _,
                h: extent.height as _,
            },
            depth: 0.0..1.0,
        };

        Ok(Self {
            adapter: window.adapter,
            buffer_memory,
            cmd_buffers,
            cmd_pools,
            frame: 0,
            image_logo,
            image_srv,
            frames_in_flight,
            image_memory,
            image_upload_buffer,
            image_upload_memory,
            queue_group,
            sampler,
            vertex_buffer,
            device,
            desc_pool,
            format,
            instance: window.instance,
            dimensions: window_dimensions.clone(),
            viewport,
            framebuffer,
            pipeline: ManuallyDrop::new(pipeline),
            pipeline_layout,
            render_pass,
            desc_set: Some(desc_set),
            set_layout,
            submission_complete_semaphores,
            submission_complete_fences,

            surface: ManuallyDrop::new(window.surface),
        })
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
impl<B: gfx_hal::Backend> Drop for GfxRenderingContext<B> {
    fn drop(&mut self) {
        self.device.wait_idle().unwrap();
        unsafe {
            self.device
                .destroy_descriptor_pool(ManuallyDrop::into_inner(ptr::read(&self.desc_pool)));
            self.device
                .destroy_descriptor_set_layout(ManuallyDrop::into_inner(ptr::read(
                    &self.set_layout,
                )));
            self.device
                .destroy_buffer(ManuallyDrop::into_inner(ptr::read(&self.vertex_buffer)));
            self.device
                .destroy_buffer(ManuallyDrop::into_inner(ptr::read(
                    &self.image_upload_buffer,
                )));
            self.device
                .destroy_image(ManuallyDrop::into_inner(ptr::read(&self.image_logo)));
            self.device
                .destroy_image_view(ManuallyDrop::into_inner(ptr::read(&self.image_srv)));
            self.device
                .destroy_sampler(ManuallyDrop::into_inner(ptr::read(&self.sampler)));
            for pool in self.cmd_pools.drain(..) {
                self.device.destroy_command_pool(pool);
            }

            for s in self.submission_complete_semaphores.drain(..) {
                self.device.destroy_semaphore(s);
            }
            for fence in self.submission_complete_fences.drain(..) {
                self.device.destroy_fence(fence);
            }
            self.device
                .destroy_render_pass(ManuallyDrop::into_inner(ptr::read(&self.render_pass)));

            self.device
                .destroy_framebuffer(ManuallyDrop::into_inner(ptr::read(&self.framebuffer)));
            self.surface.unconfigure_swapchain(&self.device);

            self.device
                .free_memory(ManuallyDrop::into_inner(ptr::read(&self.buffer_memory)));
            self.device
                .free_memory(ManuallyDrop::into_inner(ptr::read(&self.image_memory)));
            self.device
                .destroy_graphics_pipeline(ManuallyDrop::into_inner(ptr::read(&self.pipeline)));
            self.device
                .destroy_pipeline_layout(ManuallyDrop::into_inner(ptr::read(
                    &self.pipeline_layout,
                )));

            self.instance
                .destroy_surface(ManuallyDrop::into_inner(ptr::read(&self.surface)));
        }
    }
}