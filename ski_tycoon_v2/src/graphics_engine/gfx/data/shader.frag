#version 450
#extension GL_ARB_separate_shader_objects: enable
layout(location = 0) in vec2 v_uv;
layout(location = 1) in vec3 o_normal;
layout(location = 0) out vec4 target0;

layout(set=0, binding = 0) uniform texture2D u_texture;
layout(set=0, binding = 1) uniform sampler u_sampler;
layout(binding = 2) uniform SunDirection{vec3 vector;} sun_direction;
layout(binding = 3) uniform SunColor{vec4 vector;} sun_color;
vec4 onify(vec4 v){
    return v*vec4(0.0,0.0,0.0,0.0)+vec4(1.0,1.0,1.0,1.0);
}
float sun(){
    return dot(-1.0*sun_direction.vector,o_normal);
}
vec4 sun_vec(){
    float s = sun();
    return vec4(s,s,s,1.0)*onify(sun_color.vector);
}

void main(){
    target0 = sun_vec()*sun_color.vector*texture(sampler2D(u_texture,u_sampler),v_uv);
}

