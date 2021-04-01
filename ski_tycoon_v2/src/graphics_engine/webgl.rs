use super::super::prelude::Texture;
use shader::{shader_library, RuntimeAttribute};
mod shader;
use super::Mesh;
use log::{debug, error, info};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
pub use shader::{Shader, ShaderText};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlVertexArrayObject,
};
pub type RuntimeMesh = WebGlMesh;
pub type RuntimeTexture = WebGlRenderTexture;
pub type ErrorType = JsValue;
pub type Framebuffer = WebFramebuffer;

pub type InitContext = ();
pub struct RuntimeDepthTexture {
    texture: RuntimeTexture,
}
///sync can be impemented because threads do not exist in webgl
unsafe impl Sync for RuntimeMesh {}
unsafe impl Sync for RuntimeTexture {}
unsafe impl Send for RuntimeMesh {}
unsafe impl Send for RuntimeTexture {}
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
pub struct RenderingContext {
    context: WebGl2RenderingContext,
}
#[allow(dead_code)]
impl RenderingContext {
    pub fn new(_context: InitContext) -> Result<Self, ErrorType> {
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
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        Ok(Self { context })
    }
    pub fn change_viewport(&self, screen_size: &Vector2<u32>) -> Result<(), ErrorType> {
        self.context
            .viewport(0, 0, screen_size.x as i32, screen_size.y as i32);
        Ok(())
    }
    pub fn build_world_shader(&mut self) -> Result<Shader, ErrorType> {
        Ok(self.build_shader(shader_library::WORLD_SHADER)?)
    }
    /// Builds shader used for screenspace
    pub fn build_screen_shader(&mut self) -> Result<Shader, JsValue> {
        self.build_shader(shader_library::SCREEN_SHADER)
    }
    pub fn build_gui_shader(&mut self) -> Result<Shader, ErrorType> {
        info!("building gui shader");
        self.build_shader(shader_library::GUI_SHADER)
    }
    fn build_shader(&mut self, text: ShaderText) -> Result<Shader, JsValue> {
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
        info!("linked shader");
        self.context.use_program(Some(&program));
        let texture_sampler_location = self.context.get_uniform_location(&program, "u_texture");
        let mut uniforms = HashMap::new();
        for uniform_name in text.uniforms.iter() {
            info!("{}", uniform_name);
            uniforms.insert(
                uniform_name.to_string(),
                self.context.get_uniform_location(&program, uniform_name),
            );
            self.get_error();
        }
        info!("got all uniforms");
        let attributes = text
            .attributes
            .iter()
            .map(|attr| {
                self.get_error();
                (
                    attr.name.to_string(),
                    RuntimeAttribute {
                        location: Some(self.context.get_attrib_location(&program, attr.name)),
                        size: attr.size,
                    },
                )
            })
            .collect();
        info!("built shader");
        Ok(Shader {
            program,
            texture_sampler_location,
            attributes,
            uniforms,
            name: text.name,

            fragment_shader_source: text.fragment_shader,
            vertex_shader_source: text.vertex_shader,
        })
    }
    pub fn bind_shader(&mut self, shader: &Shader) -> Result<(), JsValue> {
        self.context.use_program(Some(&shader.program));
        Ok(())
    }
    pub fn build_mesh(&mut self, mesh: Mesh, shader: &Shader) -> Result<RuntimeMesh, ErrorType> {
        debug!("building mesh");
        let position_buffer = self.context.create_buffer();

        self.context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            (&position_buffer).as_ref(),
        );
        let data_size = mesh.vertex_size();
        let vao = self.context.create_vertex_array();
        self.context.bind_vertex_array(vao.as_ref());
        //  Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&mesh.vertices);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let mut addn: usize = 0;
        for desc in mesh.description.iter() {
            if !shader.attributes.contains_key(&desc.name) {
                error!("key: {} not found for shader {}", desc.name, shader.name);
                error!("{}", shader.vertex_shader_source);
                error!("{}", shader.fragment_shader_source);

                panic!("key: {} not found", desc.name);
            }
            self.context
                .enable_vertex_attrib_array(shader.attributes[&desc.name].location.unwrap() as u32);
            self.context.vertex_attrib_pointer_with_i32(
                shader.attributes[&desc.name].location.unwrap() as u32,
                desc.number_components as i32,
                WebGl2RenderingContext::FLOAT,
                false,
                data_size as i32,
                addn as i32,
            );
            addn += desc.number_components * desc.size_component;
        }
        self.get_error();
        //custom verticies

        Ok(WebGlMesh {
            vertex_array_object: vao,
            position_buffer,
            count: mesh.num_vertices() as i32,
        })
    }
    pub fn delete_mesh(&mut self, mesh: &mut WebGlMesh) -> Result<(), ErrorType> {
        self.context
            .delete_vertex_array(mesh.vertex_array_object.as_ref());
        self.context.delete_buffer(mesh.position_buffer.as_ref());
        Ok(())
    }
    pub fn send_vec3_uniform(
        &self,
        shader: &Shader,
        uniform_name: &str,
        data: Vector3<f32>,
    ) -> Result<(), ErrorType> {
        self.context.uniform3f(
            Some(&shader.uniforms[uniform_name].as_ref().unwrap()),
            data.x,
            data.y,
            data.z,
        );
        self.get_error();
        Ok(())
    }
    pub fn send_vec4_uniform(
        &self,
        shader: &Shader,
        uniform_name: &str,
        data: Vector4<f32>,
    ) -> Result<(), ErrorType> {
        self.context.uniform4f(
            Some(&shader.uniforms[uniform_name].as_ref().unwrap()),
            data.x,
            data.y,
            data.z,
            data.w,
        );
        self.get_error();
        Ok(())
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
    pub fn delete_depth_buffer(
        &mut self,
        texture: &mut RuntimeDepthTexture,
    ) -> Result<(), ErrorType> {
        self.context
            .delete_texture(texture.texture.texture.as_ref());
        Ok(())
    }
    pub fn build_texture(
        &mut self,
        texture: Texture,
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
    pub fn delete_texture(&mut self, texture: &mut RuntimeTexture) {
        self.context.delete_texture(texture.texture.as_ref())
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
    pub fn delete_framebuffer(
        &mut self,
        framebuffer: &mut WebFramebuffer,
    ) -> Result<(), ErrorType> {
        self.context
            .delete_framebuffer(framebuffer.framebuffer.as_ref());
        Ok(())
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
        self.get_error();
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
        self.get_error();
    }
    pub fn send_view_matrix(&mut self, matrix: Matrix4<f32>, shader: &Shader) {
        self.context.uniform_matrix4fv_with_f32_array(
            shader.uniforms["camera"].as_ref(),
            false,
            matrix.as_slice(),
        );
        self.get_error();
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
                .unwrap_or_else(|| String::from("Unknvhown error creating program object")))
        }
    }
    pub fn get_error(&self) {
        let e = self.context.get_error();
        if e != WebGl2RenderingContext::NO_ERROR {
            error!("error: {}", e);
            panic!()
        }
    }
}

/// Send can be implemented because threads do not exist in wasm
unsafe impl Send for RenderingContext {}
