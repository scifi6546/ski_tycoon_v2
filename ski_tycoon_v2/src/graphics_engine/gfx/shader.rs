use gfx_hal::{pso, pso::DescriptorSetLayoutBinding};
use std::collections::HashMap;
use std::io::Cursor;
pub enum ShaderTypes {
    WORLD_SHADER = 0,
}
pub struct ShaderData {
    pub fragment_shader_data: Vec<u32>,
    pub vertex_shader_data: Vec<u32>,
    pub vertex_uniform_layout: HashMap<String, DescriptorSetLayoutBinding>,
    pub fragment_uniform_layout: HashMap<String, DescriptorSetLayoutBinding>,
}
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
            "./data/shader.frag.spv"
        )))
        .unwrap(),
        vertex_shader_data: gfx_auxil::read_spirv(Cursor::new(&include_bytes!(
            "./data/shader.vert.spv"
        )))
        .unwrap(),
        vertex_uniform_layout: [
            (
                "camera".to_string(),
                DescriptorSetLayoutBinding {
                    binding: 0,
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
            ),
            (
                "model".to_string(),
                DescriptorSetLayoutBinding {
                    binding: 1,
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
            ),
        ]
        .iter()
        .map(|i| i.clone())
        .collect(),
        fragment_uniform_layout: [
            (
                "sun_direction".to_string(),
                DescriptorSetLayoutBinding {
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
            ),
            (
                "sun_color".to_string(),
                DescriptorSetLayoutBinding {
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
            ),
        ]
        .iter()
        .map(|i| i.clone())
        .collect(),
    }
}
