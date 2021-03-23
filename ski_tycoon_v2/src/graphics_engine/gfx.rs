// gfx frontent rendering engine
mod bind_arena;
mod shader;
use super::super::prelude::Texture;
use super::Mesh;
use bind_arena::{BindArena, BindArenaIndex};
use gfx_hal::{
    adapter::PhysicalDevice,
    buffer, command,
    command::CommandBuffer,
    device::Device,
    format,
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
use shader::ShaderData;

use std::collections::HashMap;
use std::{cmp::max, io::Cursor, iter, mem, mem::ManuallyDrop, ptr};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
mod circular_buffer;
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

pub type RuntimeMesh = RuntimeGfxMesh<back::Backend>;
pub type RuntimeTexture = RuntimeGfxTexture<back::Backend>;
#[derive(Clone)]
pub struct RuntimeGfxTexture<B: gfx_hal::Backend> {
    image_buffer: B::Buffer,
    image_memory: B::Memory,
    image_logo: B::Image,
    extent: gfx_hal::image::Extent,
}
#[derive(Debug)]
pub enum ErrorType {}
#[allow(dead_code)]
pub struct GfxFramebuffer<B: gfx_hal::Backend> {
    framebuffer: B::Framebuffer,
}
pub type Framebuffer = GfxFramebuffer<back::Backend>;
#[derive(Clone)]
pub struct RuntimeGfxMesh<B: gfx_hal::Backend> {
    vertex_buffer: B::Buffer,
    vertex_memory: B::Memory,
}
pub type RuntimeDepthTexture = RuntimeGfxDepthTexture<back::Backend>;
#[allow(dead_code)]
pub struct RuntimeGfxDepthTexture<B: gfx_hal::Backend> {
    image_logo: B::Image,
    image_buffer: B::Buffer,
    image_memory: B::Memory,
    extent: gfx_hal::image::Extent,
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

#[allow(dead_code)]
pub struct GfxRenderingContext<B: gfx_hal::Backend> {
    desc_pool: ManuallyDrop<B::DescriptorPool>,
    surface: ManuallyDrop<B::Surface>,
    format: gfx_hal::format::Format,
    dimensions: window::Extent2D,
    viewport: pso::Viewport,
    supported_formats: std::vec::Vec<format::Format>,
    render_pass: ManuallyDrop<B::RenderPass>,
    framebuffer: ManuallyDrop<B::Framebuffer>,
    pipeline: ManuallyDrop<B::GraphicsPipeline>,
    pipeline_layout: ManuallyDrop<B::PipelineLayout>,
    desc_set: Option<B::DescriptorSet>,
    set_layout: ManuallyDrop<B::DescriptorSetLayout>,
    submission_complete_semaphore: B::Semaphore,
    submission_complete_fence: B::Fence,
    command_pool: B::CommandPool,
    command_buffer: B::CommandBuffer,
    vertex_buffer: ManuallyDrop<B::Buffer>,
    buffer_memory: ManuallyDrop<B::Memory>,
    //should be dropped in decleration order
    device: B::Device,
    adapter: gfx_hal::adapter::Adapter<B>,
    queue_group: QueueGroup<B>,
    instance: B::Instance,
    shaders: BindArena<GfxShader<B>>,
}
pub struct Shader {
    index: BindArenaIndex,
}
#[allow(dead_code)]
struct RuntimeUniform<B: gfx_hal::Backend> {
    buffer: B::Buffer,
    memory: B::Memory,
}
#[allow(dead_code)]
pub struct GfxShader<B: gfx_hal::Backend> {
    pipeline: B::GraphicsPipeline,
    fragment_shader_uniform_buffers: HashMap<String, RuntimeUniform<B>>,
    vertex_shader_uniform_buffers: HashMap<String, RuntimeUniform<B>>,
}
pub type RenderingContext = GfxRenderingContext<back::Backend>;
pub type InitContext = Window<back::Backend>;
const DIMS: window::Extent2D = window::Extent2D {
    width: 1024,
    height: 768,
};
#[allow(dead_code, unused_variables)]
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
        let queue_group = gpu.queue_groups.pop().unwrap();
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
            println!("buffer_len: {} spirv len: {}", buffer_req.size, buffer_len);
            let mut memory = device
                .allocate_memory(upload_type, max(buffer_req.size, buffer_len))
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
        let img_data = include_bytes!("./gfx/png/Untitled.png");
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
        let caps = window.surface.capabilities(&window.adapter.physical_device);
        let supported_formats = window
            .surface
            .supported_formats(&window.adapter.physical_device);
        println!("formats: {:?}", supported_formats);
        //checking supported formats
        let format =
            supported_formats
                .clone()
                .map_or(gfx_hal::format::Format::Rgba8Srgb, |formats| {
                    formats
                        .iter()
                        .find(|format| format.base_format().1 == ChannelType::Srgb)
                        .copied()
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
        let submission_complete_semaphore = device
            .create_semaphore()
            .expect("failed to create semaphore");
        let submission_complete_fence = device.create_fence(false).expect("failed to create fence");
        // Note: We don't really need a different command pool per frame in such a simple demo like this,
        // but in a more 'real' application, it's generally seen as optimal to have one command pool per
        // thread per frame. There is a flag that lets a command pool reset individual command buffers
        // which are created from it, but by default the whole pool (and therefore all buffers in it)
        // must be reset at once. Furthermore, it is often the case that resetting a whole pool is actually
        // faster and more efficient for the hardware than resetting individual command buffers, so it's
        // usually best to just make a command pool for each set of buffers which need to be reset at the
        // same time (each frame). In our case, each pool will only have one command buffer created from it,
        // though.
        let command_buffer = unsafe { command_pool.allocate_one(command::Level::Primary) };

        let pipeline_layout = ManuallyDrop::new(
            unsafe { device.create_pipeline_layout(iter::once(&*set_layout), iter::empty()) }
                .expect("failed to create pipeline layout"),
        );
        let pipeline = {
            let vs_module = {
                let spirv = gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
                    "./gfx/compiled_shader/shader.vert.spv"
                )))
                .unwrap();
                unsafe { device.create_shader_module(&spirv).unwrap() }
            };

            let fs_module = {
                let spirv = gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
                    "./gfx/compiled_shader/shader.frag.spv"
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
            supported_formats: supported_formats.unwrap(),
            command_buffer,
            command_pool,
            queue_group,
            vertex_buffer,
            device,
            desc_pool,
            format,
            instance: window.instance,
            dimensions: window_dimensions,
            viewport,
            framebuffer,
            pipeline: ManuallyDrop::new(pipeline),
            pipeline_layout,
            render_pass,
            desc_set: Some(desc_set),
            set_layout,
            submission_complete_semaphore,
            submission_complete_fence,
            surface: ManuallyDrop::new(window.surface),
            shaders: BindArena::default(),
        })
    }
    pub fn change_viewport(&self, screen_size: &Vector2<u32>) -> Result<(), ErrorType> {
        todo!()
    }
    ///Builds shader and inserts the shaer into self.shaders
    fn build_shader(&mut self, shaders: ShaderData) -> Result<Shader, ErrorType> {
        let fragment_shader = unsafe {
            self.device
                .create_shader_module(&shaders.fragment_shader_data)
                .unwrap()
        };
        let vertex_shader = unsafe {
            self.device
                .create_shader_module(&shaders.vertex_shader_data)
                .unwrap()
        };
        let vs_entry = pso::EntryPoint {
            entry: "main",
            module: &vertex_shader,
            specialization: gfx_hal::spec_const_list![0.8f32],
        };
        let fs_entry = pso::EntryPoint {
            entry: "main",
            module: &fragment_shader,
            specialization: pso::Specialization::default(),
        };
        let subpass = Subpass {
            index: 0,
            main_pass: &*self.render_pass,
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
            &*self.pipeline_layout,
            subpass,
        );
        pipeline_desc.blender.targets.push(pso::ColorBlendDesc {
            mask: pso::ColorMask::ALL,
            blend: Some(pso::BlendState::ALPHA),
        });
        let pipeline = unsafe {
            self.device
                .create_graphics_pipeline(&pipeline_desc, None)
                .unwrap()
        };
        unsafe {
            self.device.destroy_shader_module(fragment_shader);
            self.device.destroy_shader_module(vertex_shader);
        }
        let fragment_shader_uniform_buffers = shaders
            .fragment_uniform_layout
            .iter()
            .map(|(name, uniform)| {
                let buffer = unsafe {
                    self.device
                        .create_buffer(uniform.data_type.size() as u64, buffer::Usage::UNIFORM)
                        .expect("failed to create buffer")
                };
                let memory = self.allocate_memory(&buffer);
                (name.clone(), RuntimeUniform { buffer, memory })
            })
            .collect();
        let vertex_shader_uniform_buffers = shaders
            .vertex_uniform_layout
            .iter()
            .map(|(name, uniform)| {
                let buffer = unsafe {
                    self.device
                        .create_buffer(uniform.data_type.size() as u64, buffer::Usage::UNIFORM)
                        .expect("failed to create buffer")
                };
                let memory = self.allocate_memory(&buffer);
                (name.clone(), RuntimeUniform { buffer, memory })
            })
            .collect();
        let shader = GfxShader {
            pipeline,
            fragment_shader_uniform_buffers,
            vertex_shader_uniform_buffers,
        };
        Ok(Shader {
            index: self.shaders.insert(shader),
        })
    }
    pub fn build_world_shader(&mut self) -> Result<Shader, ErrorType> {
        let shaders = shader::get_world();
        self.build_shader(shaders)
    }
    fn allocate_memory_properties(
        &mut self,
        buffer: &B::Buffer,
        properties: memory::Properties,
    ) -> B::Memory {
        let memory_types = self
            .adapter
            .physical_device
            .memory_properties()
            .memory_types;
        let buffer_reqs = unsafe { self.device.get_buffer_requirements(buffer) };
        let upload_type = memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                // type_mask is a bit field where each bit represents a memory type. If the bit is set
                // to 1 it means we can use that type for our buffer. So this code finds the first
                // memory type that has a `1` (or, is allowed), and is visible to the CPU.
                buffer_reqs.type_mask & (1 << id) != 0 && mem_type.properties.contains(properties)
            })
            .unwrap()
            .into();
        unsafe {
            self.device
                .allocate_memory(upload_type, buffer_reqs.size)
                .unwrap()
        }
    }
    fn allocate_memory(&mut self, buffer: &B::Buffer) -> B::Memory {
        self.allocate_memory_properties(buffer, memory::Properties::CPU_VISIBLE)
    }
    /// Builds shader used for screenspace
    pub fn build_screen_shader(&mut self) -> Result<Shader, ErrorType> {
        let shaders = shader::get_screen();
        self.build_shader(shaders)
    }
    pub fn build_gui_shader(&mut self) -> Result<Shader, ErrorType> {
        let shaders = shader::get_screen();
        self.build_shader(shaders)
    }
    pub fn bind_shader(&mut self, shader: &Shader) -> Result<(), ErrorType> {
        self.shaders.bind(shader.index.clone());
        Ok(())
    }
    pub fn build_mesh(
        &mut self,
        mesh: Mesh,
        shader: &Shader,
    ) -> Result<RuntimeGfxMesh<B>, ErrorType> {
        let data = mesh.to_bytes();
        let buffer = unsafe {
            self.device.create_buffer(
                data.len() as u64 * std::mem::size_of::<f32>() as u64,
                gfx_hal::buffer::Usage::VERTEX,
            )
        }
        .expect("failed to create buffer");
        let mut memory = self.allocate_memory(&buffer);
        let memory_ptr = unsafe {
            self.device.map_memory(
                &mut memory,
                gfx_hal::memory::Segment {
                    offset: 0,
                    size: None,
                },
            )
        }
        .expect("failed to map memory");
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                memory_ptr,
                data.len() * std::mem::size_of::<f32>(),
            )
        };
        unsafe {
            self.device.unmap_memory(&mut memory);
        }
        Ok(RuntimeGfxMesh {
            vertex_buffer: buffer,
            vertex_memory: memory,
        })
    }
    pub fn delete_mesh(&mut self, mesh: &mut RuntimeMesh) -> Result<(), ErrorType> {
        todo!()
    }
    unsafe fn send_uniform(
        &mut self,
        shader: &mut Shader,
        uniform_name: &str,
        data: *const u8,
        data_size: usize,
    ) {
        let shader = self.shaders.get_mut(shader.index.clone()).unwrap();
        let memory = if shader
            .fragment_shader_uniform_buffers
            .contains_key(uniform_name)
        {
            &mut shader
                .fragment_shader_uniform_buffers
                .get_mut(uniform_name)
                .unwrap()
                .memory
        } else if shader
            .vertex_shader_uniform_buffers
            .contains_key(uniform_name)
        {
            &mut shader
                .vertex_shader_uniform_buffers
                .get_mut(uniform_name)
                .unwrap()
                .memory
        } else {
            panic!("uniform: {} not found", uniform_name)
        };
        let memory_ptr = self
            .device
            .map_memory(
                memory,
                gfx_hal::memory::Segment {
                    offset: 0,
                    size: None,
                },
            )
            .expect("failed to map memory");
        std::ptr::copy_nonoverlapping(data, memory_ptr, data_size);
        self.device.unmap_memory(memory);
    }
    pub fn send_vec3_uniform(
        &mut self,
        shader: &mut Shader,
        uniform_name: &str,
        data: Vector3<f32>,
    ) -> Result<(), ErrorType> {
        unsafe {
            self.send_uniform(
                shader,
                uniform_name,
                data.as_ptr() as *const u8,
                3 * std::mem::size_of::<f32>(),
            )
        };
        Ok(())
    }
    pub fn send_vec4_uniform(
        &mut self,
        shader: &mut Shader,
        uniform_name: &str,
        data: Vector4<f32>,
    ) -> Result<(), ErrorType> {
        unsafe {
            self.send_uniform(
                shader,
                uniform_name,
                data.as_ptr() as *const u8,
                4 * std::mem::size_of::<f32>(),
            )
        };
        Ok(())
    }
    pub fn build_depth_texture(
        &mut self,
        dimensions: Vector2<u32>,
        shader: &Shader,
    ) -> Result<RuntimeGfxDepthTexture<B>, ErrorType> {
        let (image_logo, image_buffer, image_memory) = self.allocate_texture(
            Vector2::new(dimensions.x as usize, dimensions.y as usize),
            gfx_hal::image::Usage::DEPTH_STENCIL_ATTACHMENT,
        );
        let extent = gfx_hal::image::Extent {
            width: dimensions.x,
            height: dimensions.y,
            depth: 1,
        };
        Ok(RuntimeGfxDepthTexture {
            image_logo,
            image_buffer,
            image_memory,
            extent,
        })
    }
    pub fn delete_depth_buffer(
        &mut self,
        texture: &mut RuntimeDepthTexture,
    ) -> Result<(), ErrorType> {
        todo!()
    }
    fn allocate_texture(
        &mut self,
        dimensions: Vector2<usize>,
        usage: gfx_hal::image::Usage,
    ) -> (B::Image, B::Buffer, B::Memory) {
        let length = dimensions.x * dimensions.y * 4;
        let image_upload_buffer = unsafe {
            self.device
                .create_buffer(length as u64, gfx_hal::buffer::Usage::TRANSFER_SRC)
        }
        .expect("failed to create buffer");
        let memory = self.allocate_memory(&image_upload_buffer);
        let kind = gfx_hal::image::Kind::D2(
            dimensions.x as gfx_hal::image::Size,
            dimensions.y as gfx_hal::image::Size,
            1,
            1,
        );
        let image_logo = unsafe {
            self.device.create_image(
                kind,
                1,
                ColorFormat::SELF,
                gfx_hal::image::Tiling::Optimal,
                gfx_hal::image::Usage::TRANSFER_DST | usage,
                gfx_hal::image::ViewCapabilities::empty(),
            )
        }
        .unwrap();

        (image_logo, image_upload_buffer, memory)
    }
    pub fn build_texture(
        &mut self,
        texture: Texture,
        shader: &Shader,
    ) -> Result<RuntimeGfxTexture<B>, ErrorType> {
        let (mut image_logo, image_buffer, mut memory) = self.allocate_texture(
            Vector2::new(texture.width() as usize, texture.height() as usize),
            gfx_hal::image::Usage::SAMPLED,
        );
        let length = texture.pixels.len() * 4;

        let memory_ptr = unsafe {
            self.device.map_memory(
                &mut memory,
                gfx_hal::memory::Segment {
                    offset: 0,
                    size: None,
                },
            )
        }
        .expect("failed to map memory");
        unsafe {
            std::ptr::copy_nonoverlapping(texture.pixels.as_ptr() as *const u8, memory_ptr, length);
        };
        unsafe {
            self.device.unmap_memory(&mut memory);
        }
        let find_image_memory = |image: &B::Image| {
            let memory_types = self
                .adapter
                .physical_device
                .memory_properties()
                .memory_types;
            let buffer_reqs = unsafe { self.device.get_image_requirements(image) };
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    // type_mask is a bit field where each bit represents a memory type. If the bit is set
                    // to 1 it means we can use that type for our buffer. So this code finds the first
                    // memory type that has a `1` (or, is allowed), and is visible to the CPU.
                    buffer_reqs.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(memory::Properties::DEVICE_LOCAL)
                })
                .unwrap()
                .into();
            unsafe {
                self.device
                    .allocate_memory(upload_type, buffer_reqs.size)
                    .unwrap()
            }
        };
        let image_memory = find_image_memory(&image_logo);
        unsafe {
            self.device
                .bind_image_memory(&image_memory, 0, &mut image_logo)
                .expect("failed to bind image memory");
        }

        //copying image
        unsafe {
            self.command_buffer
                .begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
            let image_barrier = gfx_hal::memory::Barrier::Image {
                states: (
                    gfx_hal::image::Access::empty(),
                    gfx_hal::image::Layout::Undefined,
                )
                    ..(
                        gfx_hal::image::Access::TRANSFER_WRITE,
                        gfx_hal::image::Layout::TransferDstOptimal,
                    ),
                target: &image_logo,
                families: None,
                range: gfx_hal::image::SubresourceRange {
                    aspects: gfx_hal::format::Aspects::COLOR,
                    ..Default::default()
                },
            };
            self.command_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                gfx_hal::memory::Dependencies::empty(),
                iter::once(image_barrier),
            );
            self.command_buffer.copy_buffer_to_image(
                &image_buffer,
                &image_logo,
                gfx_hal::image::Layout::TransferDstOptimal,
                iter::once(command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: texture.width() * 4,
                    buffer_height: texture.height(),
                    image_layers: gfx_hal::image::SubresourceLayers {
                        aspects: gfx_hal::format::Aspects::COLOR,
                        level: 0,
                        layers: 0..1,
                    },
                    image_offset: gfx_hal::image::Offset { x: 0, y: 0, z: 0 },
                    image_extent: gfx_hal::image::Extent {
                        width: texture.width(),
                        height: texture.height(),
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
                target: &image_logo,
                families: None,
                range: gfx_hal::image::SubresourceRange {
                    aspects: gfx_hal::format::Aspects::COLOR,
                    ..Default::default()
                },
            };
            self.command_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                gfx_hal::memory::Dependencies::empty(),
                iter::once(image_barrier),
            );
            self.command_buffer.finish();
            if self
                .device
                .get_fence_status(&self.submission_complete_fence)
                .expect("failed to get fence")
            {
                panic!("fence not reset at submission of command queue");
            } else {
                println!("fence properly reset");
            }
            self.queue_group.queues[0].submit(
                iter::once(&self.command_buffer),
                iter::empty(),
                iter::empty(),
                Some(&mut self.submission_complete_fence),
            );
            self.device
                .wait_for_fence(&self.submission_complete_fence, !0)
                .expect("failed to wait for fence");
            self.device
                .reset_fence(&mut self.submission_complete_fence)
                .expect("failed to reset fence");
        }
        unsafe {
            self.device.free_memory(memory);
        }
        let extent = gfx_hal::image::Extent {
            width: texture.dimensions.x,
            height: texture.dimensions.y,
            depth: 1,
        };
        Ok(RuntimeGfxTexture {
            image_buffer,
            image_memory,
            image_logo,
            extent,
        })
    }
    pub fn delete_texture(&mut self, texture: &mut RuntimeTexture) {
        todo!()
    }
    pub fn build_framebuffer(
        &mut self,
        texture_attachment: &mut RuntimeGfxTexture<B>,
        depth_attachment: &mut RuntimeGfxDepthTexture<B>,
    ) -> Result<GfxFramebuffer<B>, ErrorType> {
        assert_eq!(texture_attachment.extent, depth_attachment.extent);
        let framebuffer = unsafe {
            self.device.create_framebuffer(
                &self.render_pass,
                [
                    gfx_hal::image::FramebufferAttachment {
                        usage: gfx_hal::image::Usage::TRANSFER_DST
                            | gfx_hal::image::Usage::COLOR_ATTACHMENT,
                        view_caps: gfx_hal::image::ViewCapabilities::empty(),
                        format: ColorFormat::SELF,
                    },
                    gfx_hal::image::FramebufferAttachment {
                        usage: gfx_hal::image::Usage::TRANSFER_DST
                            | gfx_hal::image::Usage::DEPTH_STENCIL_ATTACHMENT,
                        view_caps: gfx_hal::image::ViewCapabilities::empty(),
                        format: ColorFormat::SELF,
                    },
                ]
                .iter()
                .cloned(),
                texture_attachment.extent,
            )
        }
        .expect("failed to create framebuffer");
        Ok(GfxFramebuffer { framebuffer })
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
    /// Checks all preconditions,
    /// currently the only precondition is that the fence must always be unsignaled
    pub fn get_error(&mut self) {
        assert_eq!(
            unsafe {
                self.device
                    .get_fence_status(&self.submission_complete_fence)
            }
            .expect("failed to get fence status"),
            false
        );
        //nothing to do here
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
            // self.device.destroy_command_pool(self.cmd_buffer.0);

            //self.device
            //    .destroy_semaphore(self.submission_complete_semaphore);
            //self.device.destroy_fence(self.submission_complete_fence);
            self.device
                .destroy_render_pass(ManuallyDrop::into_inner(ptr::read(&self.render_pass)));

            self.device
                .destroy_framebuffer(ManuallyDrop::into_inner(ptr::read(&self.framebuffer)));
            self.surface.unconfigure_swapchain(&self.device);

            self.device
                .free_memory(ManuallyDrop::into_inner(ptr::read(&self.buffer_memory)));
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
