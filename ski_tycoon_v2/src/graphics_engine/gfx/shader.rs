use gfx_hal::{pso, pso::DescriptorSetLayoutBinding};
use std::collections::HashMap;
use std::io::Cursor;
#[allow(dead_code)]
pub enum ShaderTypes {
    WorldShader = 0,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum UniformDataType {
    Vec3,
    Vec4,
    Mat3,
    Mat4,
}
impl UniformDataType {
    pub fn size(&self) -> usize {
        match self {
            Self::Vec3 => 3 * std::mem::size_of::<f32>(),
            Self::Vec4 => 4 * std::mem::size_of::<f32>(),
            Self::Mat3 => 3 * 3 * std::mem::size_of::<f32>(),
            Self::Mat4 => 4 * 4 * std::mem::size_of::<f32>(),
        }
    }
}
#[derive(Clone)]
pub struct UniformData {
    pub layout_binding: DescriptorSetLayoutBinding,
    pub data_type: UniformDataType,
}
pub struct ShaderData {
    pub fragment_shader_data: Vec<u32>,
    pub vertex_shader_data: Vec<u32>,
    pub vertex_uniform_layout: HashMap<String, UniformData>,
    pub fragment_uniform_layout: HashMap<String, UniformData>,
}
#[allow(dead_code)]
fn get_vec(data: &'static [u8]) -> Vec<u32> {
    assert_eq!(data.len() % 4, 0);
    let mut out = vec![];
    out.reserve(data.len() % 4);
    for i in 0..data.len() / 4 {
        let bytes: Vec<u8> = (0..4).map(|j| data[i * 4 + j]).collect();
        out.push(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
    }

    out
}
pub fn get_world() -> ShaderData {
    ShaderData {
        fragment_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/shader.frag.spv"
        )))
        .unwrap(),
        vertex_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/shader.vert.spv"
        )))
        .unwrap(),
        vertex_uniform_layout: [
            (
                "camera".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
            (
                "model".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        fragment_uniform_layout: [
            (
                "sun_direction".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 2,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::FRAGMENT,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Vec3,
                },
            ),
            (
                "sun_color".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 3,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::FRAGMENT,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Vec4,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    }
}
pub fn get_screen() -> ShaderData {
    ShaderData {
        fragment_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/screen.frag.spv"
        )))
        .unwrap(),
        vertex_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/screen.vert.spv"
        )))
        .unwrap(),
        vertex_uniform_layout: [
            (
                "camera".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
            (
                "model".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        fragment_uniform_layout: HashMap::new(),
    }
}
#[allow(dead_code)]
pub fn get_gui() -> ShaderData {
    ShaderData {
        fragment_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/gui.frag.spv"
        )))
        .unwrap(),
        vertex_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./compiled_shader/gui.vert.spv"
        )))
        .unwrap(),
        vertex_uniform_layout: [
            (
                "camera".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
            (
                "model".to_string(),
                UniformData {
                    layout_binding: DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::Buffer {
                            ty: pso::BufferDescriptorType::Uniform,
                            format: pso::BufferDescriptorFormat::Structured {
                                dynamic_offset: false,
                            },
                        },
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::VERTEX,
                        immutable_samplers: false,
                    },
                    data_type: UniformDataType::Mat4,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        fragment_uniform_layout: HashMap::new(),
    }
}
