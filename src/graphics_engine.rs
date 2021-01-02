mod mesh;
mod shader;
use log::{debug, error};
pub use mesh::{Mesh, Vertex};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
pub use shader::Shader;
use shader::ShaderLibrary;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};
#[derive(Debug, Clone)]
pub struct Transform {
    scaling: Vector3<f32>,
    translation: Vector3<f32>,
}
impl Transform {
    ///Adds a delta translation
    pub fn translate(&mut self, delta: Vector3<f32>) {
        self.translation += delta;
    }
    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.translation = translation;
    }
    pub fn scale(&mut self, delta: Vector3<f32>) {
        self.scaling += delta;
    }
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scaling = scale;
    }
    pub fn build(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.translation) * Matrix4::new_nonuniform_scaling(&self.scaling)
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            scaling: Vector3::new(1.0, 1.0, 1.0),
            translation: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
pub type RuntimeMesh = WebGlMesh;
pub type RuntimeTexture = WebGlRenderTexture;
pub type ErrorType = JsValue;
pub type Framebuffer = WebFramebuffer;
pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
///sync can be impemented because threads do not exist in webgl
unsafe impl Sync for RuntimeMesh {}
unsafe impl Sync for RuntimeTexture {}
unsafe impl Send for RuntimeMesh {}
unsafe impl Send for RuntimeTexture {}
#[derive(Clone)]
pub struct RGBATexture {
    dimensions: Vector2<u32>,
    pixels: Vec<Vector4<u8>>,
}
impl RGBATexture {
    pub fn get_raw_vector(&self) -> Vec<u8> {
        let mut v = vec![];
        v.reserve((self.dimensions.x * self.dimensions.y * 4) as usize);
        for pixel in self.pixels.iter() {
            v.push(pixel.x);
            v.push(pixel.y);
            v.push(pixel.z);
            v.push(pixel.w);
        }
        return v;
    }
    pub fn constant_color(color: Vector4<u8>, dimensions: Vector2<u32>) -> Self {
        let pixels = (0..(dimensions.x * dimensions.y))
            .map(|_| color.clone())
            .collect();
        Self { dimensions, pixels }
    }
}
pub struct WebGl {
    context: WebGl2RenderingContext,
}
#[derive(Clone)]
pub struct WebGlMesh {
    vertex_array_object: Option<WebGlVertexArrayObject>,
    position_buffer: Option<WebGlBuffer>,
    count: i32,
}
#[derive(Clone)]
pub struct WebGlRenderTexture {
    texture: Option<WebGlTexture>,
}
pub struct WebFramebuffer {
    framebuffer: Option<WebGlFramebuffer>,
}
impl WebGl {
    pub fn new() -> Result<Self, ErrorType> {
        debug!("creating webgl2 instance");
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        let vert_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"#version 300 es
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
        )?;
        let frag_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r#"#version 300 es
        precision highp float;
        out vec4 color;
        in vec2 o_uv;
        in vec3 o_normal;
        uniform sampler2D u_texture;
        void main() {
            color = texture(u_texture,o_uv)+vec4(o_normal,1.0);
        }
    "#,
        )?;
        let program = Self::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let uv_attribute_location = context.get_attrib_location(&program, "uv");
        let normal_attribute_location = context.get_attrib_location(&program, "normal");
        let texture_sampler_location = context.get_uniform_location(&program, "u_texture");
        Ok(Self { context })
    }
    pub fn build_world_shader(&mut self) -> Result<Shader, ErrorType> {
        let text = ShaderLibrary::WORLD_SHADER;
        let vertex_shader = Self::compile_shader(
            &self.context,
            WebGl2RenderingContext::VERTEX_SHADER,
            text.vertex_shader,
        )?;
        let frag_shader = Self::compile_shader(
            &self.context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            text.fragment_shader,
        )?;
        let program = Self::link_program(&self.context, &vertex_shader, &frag_shader)?;
        self.context.use_program(Some(&program));
        let position_attribute_location =
            Some(self.context.get_attrib_location(&program, "position"));
        let uv_attribute_location = Some(self.context.get_attrib_location(&program, "uv"));
        let normal_attribute_location = Some(self.context.get_attrib_location(&program, "normal"));
        let texture_sampler_location = self.context.get_uniform_location(&program, "u_texture");
        let mut uniforms = HashMap::new();
        uniforms.insert(
            "model".to_string(),
            self.context.get_uniform_location(&program, "model"),
        );
        uniforms.insert(
            "camera".to_string(),
            self.context.get_uniform_location(&program, "camera"),
        );
        Ok(Shader {
            program,
            position_attribute_location,
            uv_attribute_location,
            normal_attribute_location,
            texture_sampler_location,
            uniforms,
        })
    }
    pub fn build_mesh(&mut self, mesh: Mesh, shader: &Shader) -> Result<RuntimeMesh, ErrorType> {
        debug!("building mesh");
        let position_buffer = self.context.create_buffer();
        let mut array: Vec<f32> = vec![];

        self.context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            (&position_buffer).as_ref(),
        );
        for vertex in mesh.vertices.iter() {
            array.push(vertex.position.x);
            array.push(vertex.position.y);
            array.push(vertex.position.z);
            array.push(vertex.uv.x);
            array.push(vertex.uv.y);
            array.push(vertex.normal.x);
            array.push(vertex.normal.y);
            array.push(vertex.normal.z);
        }
        //  Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&array);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let vao = self.context.create_vertex_array();
        self.context.bind_vertex_array(vao.as_ref());
        self.context
            .enable_vertex_attrib_array(shader.position_attribute_location.unwrap() as u32);
        self.context
            .enable_vertex_attrib_array(shader.uv_attribute_location.unwrap() as u32);
        self.context
            .enable_vertex_attrib_array(shader.normal_attribute_location.unwrap() as u32);
        self.context.vertex_attrib_pointer_with_f64(
            shader.position_attribute_location.unwrap() as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            (3 + 2 + 3) * std::mem::size_of::<f32>() as i32,
            0.0,
        );
        self.context.vertex_attrib_pointer_with_i32(
            shader.uv_attribute_location.unwrap() as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            (3 + 2 + 3) * std::mem::size_of::<f32>() as i32,
            3 * std::mem::size_of::<f32>() as i32,
        );
        self.context.vertex_attrib_pointer_with_i32(
            shader.normal_attribute_location.unwrap() as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            (3 + 2 + 3) * std::mem::size_of::<f32>() as i32,
            5 * std::mem::size_of::<f32>() as i32,
        );
        Ok(WebGlMesh {
            vertex_array_object: vao,
            position_buffer,
            count: mesh.vertices.len() as i32,
        })
    }
    pub fn build_depth_texture(
        &mut self,
        dimensions: Vector2<u32>,
        shader: &Shader,
    ) -> Result<RuntimeDepthTexture, ErrorType> {
        debug!("building texture");
        let gl_texture = self.context.create_texture();
        assert!(gl_texture.is_some());
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, gl_texture.as_ref());
        let texture_unit = 0;
        self.context
            .active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit);
        let level = 0;
        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                level,
                //  Use RGBA Format
                WebGl2RenderingContext::DEPTH_COMPONENT24 as i32,
                //width
                dimensions.x as i32,
                //height
                dimensions.y as i32,
                //must be 0 specifies the border
                0,
                //  Use RGB Format
                WebGl2RenderingContext::DEPTH_COMPONENT,
                WebGl2RenderingContext::UNSIGNED_INT,
                None,
            )?;
        //self.gl_context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        //getting location of sampler

        self.context.uniform1i(
            shader.texture_sampler_location.as_ref(),
            texture_unit as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        Ok(RuntimeDepthTexture {
            texture: WebGlRenderTexture {
                texture: gl_texture,
            },
        })
    }
    pub fn build_texture(
        &mut self,
        texture: RGBATexture,
        shader: &Shader,
    ) -> Result<RuntimeTexture, ErrorType> {
        debug!("building texture");
        let gl_texture = self.context.create_texture();
        assert!(gl_texture.is_some());
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, gl_texture.as_ref());
        let texture_unit = 0;
        self.context
            .active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit);
        let level = 0;
        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                level,
                //  Use RGBA Format
                WebGl2RenderingContext::RGBA as i32,
                //width
                texture.dimensions.x as i32,
                //height
                texture.dimensions.y as i32,
                //must be 0 specifies the border
                0,
                //  Use RGB Format
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                texture.get_raw_vector().as_slice(),
                0,
            )?;
        //self.gl_context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        //getting location of sampler

        self.context.uniform1i(
            shader.texture_sampler_location.as_ref(),
            texture_unit as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        Ok(WebGlRenderTexture {
            texture: gl_texture,
        })
    }
    pub fn build_framebuffer(
        &mut self,
        texture_attachment: &mut RuntimeTexture,
        depth_attachment: &mut RuntimeDepthTexture,
    ) -> Result<Framebuffer, ErrorType> {
        debug!("building framebuffer");
        let framebuffer = self.context.create_framebuffer();
        self.context
            .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer.as_ref());
        self.context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            texture_attachment.texture.as_ref(),
            0,
        );
        self.context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::TEXTURE_2D,
            depth_attachment.texture.texture.as_ref(),
            0,
        );
        if self
            .context
            .check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER)
            != WebGl2RenderingContext::FRAMEBUFFER_COMPLETE
        {
            error!("Framebuffer not complete");
            panic!();
        }
        // rebinding to default framebuffer to prevent side effects
        self.bind_default_framebuffer();
        Ok(WebFramebuffer { framebuffer })
    }
    pub fn bind_default_framebuffer(&mut self) {
        debug!("binding default framebuffer");
        self.context
            .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    }
    pub fn clear_screen(&mut self, color: Vector4<f32>) {
        self.context.clear_depth(1.0);
        self.context.depth_func(WebGl2RenderingContext::LESS);
        self.context.clear_color(color.x, color.y, color.z, color.w);
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }
    pub fn clear_depth(&mut self) {
        self.context.clear_depth(1.0);
        self.context.depth_func(WebGl2RenderingContext::LESS);
        self.context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }
    pub fn bind_texture(&mut self, texture: &RuntimeTexture, shader: &Shader) {
        debug!("binding texture");
        self.context
            .active_texture(WebGl2RenderingContext::TEXTURE0);
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.texture.as_ref());
        self.context
            .uniform1i(shader.texture_sampler_location.as_ref(), 0);
    }
    pub fn bind_framebuffer(&mut self, framebuffer: &Framebuffer) {
        debug!("binding framebuffer");
        self.context.bind_framebuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            framebuffer.framebuffer.as_ref(),
        );
    }
    pub fn draw_mesh(&mut self, mesh: &RuntimeMesh) {
        debug!("drawing mesh");
        self.context
            .bind_vertex_array(mesh.vertex_array_object.as_ref());
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, mesh.count);
    }
    pub fn draw_lines(&mut self, mesh: &RuntimeMesh) {
        debug!("drawing lines");
        self.context
            .bind_vertex_array(mesh.vertex_array_object.as_ref());
        self.context
            .draw_arrays(WebGl2RenderingContext::LINES, 0, mesh.count);
    }
    pub fn send_model_matrix(&mut self, matrix: Matrix4<f32>, shader: &Shader) {
        self.context.uniform_matrix4fv_with_f32_array(
            shader.uniforms["model"].as_ref(),
            false,
            matrix.as_slice(),
        );
    }
    pub fn send_view_matrix(&mut self, matrix: Matrix4<f32>, shader: &Shader) {
        debug!("sending view matrix");
        self.context.uniform_matrix4fv_with_f32_array(
            shader.uniforms["camera"].as_ref(),
            false,
            matrix.as_slice(),
        );
    }
    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
/// Send can be implemented because threads do not exist in wasm
unsafe impl Send for WebGl {}
