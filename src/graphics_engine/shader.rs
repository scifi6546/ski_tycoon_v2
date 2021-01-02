use std::collections::HashMap;
use web_sys::{WebGlProgram, WebGlUniformLocation};
pub struct ShaderText {
    pub fragment_shader: &'static str,
    pub vertex_shader: &'static str,
}
pub mod ShaderLibrary {
    use super::ShaderText;
    pub const WORLD_SHADER: ShaderText = ShaderText {
        vertex_shader: r#"#version 300 es
        in vec3 position;
        in vec2 uv;
        in vec3 normal;
        out vec2 o_uv;
        out vec3 o_normal;
        uniform mat4 camera;
        uniform mat4 model;
        void main() {
            gl_Position = camera*model*vec4(position,1.0);
            o_normal = normal;
            o_uv = uv;
        }
    "#,
        fragment_shader: r#"#version 300 es
        precision highp float;
        out vec4 color;
        in vec2 o_uv;
        in vec3 o_normal;
        uniform sampler2D u_texture;
        void main() {
            color = texture(u_texture,o_uv)+vec4(o_normal,1.0);
        }
    "#,
    };
}
pub struct Shader {
    pub uniforms: HashMap<String, Option<WebGlUniformLocation>>,
    pub position_attribute_location: Option<i32>,
    pub uv_attribute_location: Option<i32>,
    pub normal_attribute_location: Option<i32>,
    pub texture_sampler_location: Option<WebGlUniformLocation>,
    pub program: WebGlProgram,
}
/// safe because threads do not exist on wasm
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}
