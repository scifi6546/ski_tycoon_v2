use std::collections::HashMap;
use web_sys::{WebGlProgram, WebGlUniformLocation};
pub struct ShaderText {
    pub fragment_shader: &'static str,
    pub vertex_shader: &'static str,
}
pub mod shader_library {
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
        uniform vec3 sun_direction;
        uniform vec4 sun_color;
        uniform sampler2D u_texture;
        vec4 onify(vec4 v){
            return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
        }
        float sun(){
            return dot(-1.0*sun_direction,o_normal);
        }
        vec4 sun_vec(){
            float s = sun();
            return vec4(s,s,s,1.0)*onify(sun_color);
        }
        void main() {
            color = sun_vec()*sun_color*texture(u_texture,o_uv);
        }
    "#,
    };
    pub const SCREEN_SHADER: ShaderText = ShaderText {
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
            color = texture(u_texture,o_uv);
        }
    "#,
    };
}
#[derive(Clone)]
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
