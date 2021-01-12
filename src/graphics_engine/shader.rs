use std::collections::HashMap;
use web_sys::{WebGlProgram, WebGlUniformLocation};
pub struct Attribute {
    /// number of floats in attribute
    pub size: usize,
    pub name: &'static str,
}
pub struct ShaderText {
    pub fragment_shader: &'static str,
    pub vertex_shader: &'static str,
    pub uniforms: &'static [&'static str],
    pub custom_attributes: &'static [Attribute],
}
pub mod shader_library {
    use super::{Attribute, ShaderText};
    pub const GUI_SHADER: ShaderText = ShaderText {
        vertex_shader: r#"#version 300 es
        in vec3 position;
        in vec2 uv;
        in vec3 normal;
        in vec4 vertex_color;
        out vec2 o_uv;
        out vec3 o_normal;
        out vec4 o_color;
        uniform mat4 camera;
        uniform mat4 model;
        void main() {
            gl_Position = vec4(position,1.0);
            o_normal = normal;
            o_uv = uv;
            o_color=vertex_color;
        }
    "#,
        fragment_shader: r#"#version 300 es
        precision highp float;
        
        out vec4 color;
        in vec2 o_uv;
        in vec3 o_normal;
        in vec4 o_color;
        uniform sampler2D u_texture;
        vec4 onify(vec4 v){
            return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
        }
        vec4 one_alpha(vec4 v){
            v.w=1.0;
            return v;
        }
        void main() {
            color = one_alpha(o_color*10.0);
        }
    "#,
        uniforms: &[],
        custom_attributes: &[Attribute {
            size: 4,
            name: "vertex_color",
        }],
    };
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
        uniforms: &["camera", "model", "sun_direction", "sun_color"],
        custom_attributes: &[],
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
        uniforms: &["camera", "model"],
        custom_attributes: &[],
    };
}
#[derive(Clone)]
pub struct RuntimeAttribute {
    pub location: Option<i32>,
    pub size: usize,
}
#[derive(Clone)]
pub struct Shader {
    pub uniforms: HashMap<String, Option<WebGlUniformLocation>>,
    pub position_attribute_location: Option<i32>,
    pub uv_attribute_location: Option<i32>,
    pub normal_attribute_location: Option<i32>,
    pub attributes: HashMap<String, RuntimeAttribute>,
    pub texture_sampler_location: Option<WebGlUniformLocation>,
    pub program: WebGlProgram,
}
/// safe because threads do not exist on wasm
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}
