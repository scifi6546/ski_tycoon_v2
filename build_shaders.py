#!/usr/bin/env python3
from os import listdir
import subprocess
shader_dir = "ski_tycoon_v2/src/graphics_engine/gfx/data"
out_dir = "ski_tycoon_v2/src/graphics_engine/gfx/compiled_shader/"
for shader in listdir(shader_dir):
    shader_path = shader_dir+"/"+shader
    print(shader_path)
    compiled_shader_path = out_dir+shader+".spv"
    subprocess.run(["glslc","-o",compiled_shader_path,shader_path])
