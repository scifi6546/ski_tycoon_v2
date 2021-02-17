#version 450
#extension GL_ARB_separate_shader_objects: enable
layout(location = 0) in vec2 v_uv;
layout(location = 1) in vec3 o_normal;
layout(location = 2) in vec4 o_color;
layout(location = 0) out vec4 target0;

layout(set=0, binding = 0) uniform texture2D u_texture;
layout(set=0, binding = 1) uniform sampler u_sampler;
vec4 onify(vec4 v){
    return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
}
vec4 one_alpha(vec4 v){
    v.w=1.0;
    return v;
}
void main(){
    target0 = one_alpha(o_color*10.0)*one_alpha(texture(sampler2D(u_texture,u_sampler),v_uv));
}

