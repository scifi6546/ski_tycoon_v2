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
    pub attributes: &'static [Attribute],
    pub name: &'static str,
}
pub mod shader_library {
    use super::{Attribute, ShaderText};
    pub const GUI_SHADER: ShaderText = ShaderText {
        name: "GUI_SHADER",
        vertex_shader: r#"#version 300 es
        in vec3 position;
        in vec2 uv;
        in vec4 vertex_color;
        out vec2 o_uv;
        out vec4 o_color;
        uniform mat4 camera;
        uniform mat4 model;
        void main() {
            gl_Position = vec4(position,1.0);
            o_uv = uv;
            o_color=vertex_color;
        }
    "#,
        fragment_shader: r#"#version 300 es
        precision highp float;
        
        out vec4 color;
        in vec2 o_uv;
        in vec4 o_color;
        uniform sampler2D u_texture;
        vec4 onify(vec4 v){
            return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
        }
        // 0-255 sRGB  from  0-1 linear
        vec3 srgb_from_linear(vec3 rgb) {
          bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
          vec3 lower = rgb * vec3(3294.6);
          vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
          return mix(higher, lower, vec3(cutoff));
        }

        // 0-255 sRGBA  from  0-1 linear
        vec4 srgba_from_linear(vec4 rgba) {
          return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
        }
        vec4 one_alpha(vec4 v){
            v.w=1.0;
            return v;
        }
        void main() {
            color = srgba_from_linear(o_color*texture(u_texture,o_uv))/255.0;
        }

    "#,
        uniforms: &[],
        attributes: &[
            Attribute {
                size: 3,
                name: "position",
            },
            Attribute {
                size: 2,
                name: "uv",
            },
            Attribute {
                size: 4,
                name: "vertex_color",
            },
        ],
    };
    pub const WORLD_SHADER: ShaderText = ShaderText {
        name: "WORLD_SHADER",
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
        attributes: &[
            Attribute {
                size: 3,
                name: "position",
            },
            Attribute {
                size: 2,
                name: "uv",
            },
            Attribute {
                size: 3,
                name: "normal",
            },
        ],
    };
    pub const SCREEN_SHADER: ShaderText = ShaderText {
        name: "SCREEN_SHADER",
        vertex_shader: r#"#version 300 es
        in vec3 position;
        in vec2 uv;
        out vec2 o_uv;
        uniform mat4 camera;
        uniform mat4 model;
        void main() {
            gl_Position = camera*model*vec4(position,1.0);
            o_uv = uv;
        }
    "#,
        fragment_shader: r#"#version 300 es
        precision highp float;
        
        out vec4 color;
        in vec2 o_uv;
        uniform sampler2D u_texture;
        void main() {
            color = texture(u_texture,o_uv);
        }
    "#,
        uniforms: &["camera", "model"],
        attributes: &[
            Attribute {
                size: 3,
                name: "position",
            },
            Attribute {
                size: 2,
                name: "uv",
            },
        ],
    };
}
#[derive(Clone, Debug)]
pub struct RuntimeAttribute {
    pub location: Option<i32>,
    pub size: usize,
}
#[derive(Clone, Debug)]
pub struct Shader {
    pub uniforms: HashMap<String, Option<WebGlUniformLocation>>,
    pub attributes: HashMap<String, RuntimeAttribute>,
    pub texture_sampler_location: Option<WebGlUniformLocation>,
    pub fragment_shader_source: &'static str,
    pub vertex_shader_source: &'static str,
    pub name: &'static str,
    pub program: WebGlProgram,
}
/// safe because threads do not exist on wasm
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}
