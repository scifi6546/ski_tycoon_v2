pub enum ShaderTypes {
    WORLD_SHADER = 0,
}
pub struct ShaderData {
    pub fragment_shader_data: Vec<u32>,
    pub vertex_shader_data: Vec<u32>,
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
        fragment_shader_data: get_vec(include_bytes!("compiled_shader/world.frag.spv")),
        vertex_shader_data: get_vec(include_bytes!("compiled_shader/world.vert.spv")),
    }
}
