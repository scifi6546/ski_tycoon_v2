#version 450
#extension GL_ARB_separate_shader_objects: enable
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 normal;
layout(location = 0) out vec2 o_uv;
layout(location=1) out vec3 o_normal;

layout(binding = 0) uniform Camera{ mat4 matrix;} camera;
layout(binding = 1) uniform Model{mat4 matrix;} model;


out gl_PerVertex{
    vec4 gl_Position;
};
void main(){
    o_uv = uv;
    o_normal = normal;
    gl_Position = camera.matrix*model.matrix*vec4(1.0*position,1.0);

}
