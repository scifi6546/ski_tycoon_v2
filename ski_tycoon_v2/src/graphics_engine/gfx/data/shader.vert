#version 450
#extension GL_ARB_separate_shader_objects: enable
layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 u_uv;
layout(location = 0) out vec2 v_uv;

out gl_PerVertex{
    vec4 gl_Position;
};
void main(){
    v_uv = u_uv;
    gl_Position = vec4(1.2*pos,1.0);

}
