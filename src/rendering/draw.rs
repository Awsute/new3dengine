extern crate gl33;
extern  crate bytemuck;
use super::uniform::*;
use std::{ops::Deref, str::from_utf8};
use crate::{scene::*, engine::client::Client};
use sdl2::{image::*, surface::Surface};
use gl33::{*, gl_core_types::*, gl_enumerations::*, gl_groups::*, global_loader::*};
use glm::{*};

use crate::scene::object::Model;

pub fn buffer_data(gl:&GlFns, ty: GLenum, data: &[u8], usage: GLenum) {
    unsafe {
      gl.BufferData(
        ty,
        data.len().try_into().unwrap(),
        data.as_ptr().cast(),
        usage,
      );
    }
}
const SIZE_OF_F32 : usize = std::mem::size_of::<f32>();
pub unsafe fn shader_program(gl : &GlFns, vert_shader : &String, frag_shader : &String) -> u32 {
    let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);


    gl.ShaderSource(
        vertex_shader,
        1,
        &(vert_shader.as_bytes().as_ptr().cast()),
        &(vert_shader.len().try_into().unwrap()),
    );

    gl.CompileShader(vertex_shader);





    let mut success = 0;
    gl.GetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
    
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl.GetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }


    let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);

    gl.ShaderSource(
        fragment_shader,
        1,
        &(frag_shader.as_bytes().as_ptr().cast()),
        &(frag_shader.len().try_into().unwrap()),
    );
    gl.CompileShader(fragment_shader);

    let mut success = 0;
    gl.GetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl.GetShaderInfoLog(
            fragment_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }
    
    
    let shader_program = gl.CreateProgram();
    gl.AttachShader(shader_program, vertex_shader);
    gl.AttachShader(shader_program, fragment_shader);

    gl.LinkProgram(shader_program);


    gl.UseProgram(shader_program);
    return shader_program
}

pub unsafe fn delete_shader_program(gl : &GlFns, shader_program : u32) {
    gl.DeleteProgram(shader_program);
}

pub unsafe fn load_vertex_attributes(gl : &GlFns) {
    let stride = (8*SIZE_OF_F32).try_into().unwrap();
    
    //Position
    gl.VertexAttribPointer(
        0,
        3,
        GL_FLOAT,
        0,
        stride,
        0 as *const _,
    );

    gl.EnableVertexAttribArray(0);


    

    //Normals
    gl.VertexAttribPointer(
        1,
        3,
        GL_FLOAT,
        0,
        stride,
        (3*SIZE_OF_F32) as *const _
    );
    gl.EnableVertexAttribArray(1);


    
    
    //Texture Coords
    gl.VertexAttribPointer(
        2,
        2,
        GL_FLOAT,
        0,
        stride,
        (6*SIZE_OF_F32) as *const _
    );
    gl.EnableVertexAttribArray(2);

}

pub unsafe fn generate_vertex_array(gl : &GlFns) {
    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    gl.BindVertexArray(vao);
}

pub unsafe fn generate_buffers(gl : &GlFns, vertices : &Vec<f32>, indices : &Vec<u32>){
    
    let mut vbo = 0;
    let mut ebo = 0;
    gl.GenBuffers(1, &mut vbo);
    gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

    buffer_data(
        gl,
        GL_ARRAY_BUFFER, 
        bytemuck::cast_slice(vertices.as_slice()), 
        GL_STATIC_DRAW
    );

    gl.GenBuffers(1, &mut ebo);
    gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

    buffer_data(
        gl,
        GL_ELEMENT_ARRAY_BUFFER,
        bytemuck::cast_slice(indices.as_slice()), 
        GL_STATIC_DRAW
    );
}

pub unsafe fn generate_texture_data(gl : &GlFns, texture_data : Surface){
    let mut tex = 0;
        
    gl.GenTextures(1, &mut tex);
    gl.BindTexture(GL_TEXTURE_2D, tex);
    gl.TexImage2D(
        GL_TEXTURE_2D, 
        0, 
        GL_RGB.0.try_into().unwrap(), 
        texture_data.width() as i32, 
        texture_data.height() as i32, 
        0, 
        GL_RGB, 
        GL_UNSIGNED_BYTE, 
        texture_data.without_lock().unwrap().as_ptr() as *const _
    );
    gl.GenerateMipmap(GL_TEXTURE_2D);
}
pub fn draw_object(client : &Client, object : &Model, vert_shader : &String, frag_shader : &String){
    unsafe {
        let gl = &client.gl;
        gl.PolygonMode(GL_FRONT_AND_BACK, GL_TRIANGLES);
        
        let camera = &client.camera;
        let server = &client.server;
        let indices = &object.mesh.index_buffer;
        let vertices = &object.mesh.vertex_buffer;
        let texture_data = sdl2::surface::Surface::from_file(object.texture).unwrap();

        let shader_program = shader_program(gl, vert_shader, frag_shader);

        let mvp_obj = object.view_obj.look_at_up(object.view_obj.up).try_inverse().unwrap();
        let look_at = camera.view_obj.look_at();
        let projection = camera.projection;
        
        
        
        uniform_matrix4(gl, shader_program, "mvp", mvp_obj.as_ptr());
        uniform_matrix4(gl, shader_program, "lookAt", look_at.as_ptr());
        uniform_matrix4(gl, shader_program, "projection", projection.as_ptr());
        
        
        uniform_vec3(gl, shader_program, "cameraDirection", camera.view_obj.forward.as_ptr());
        uniform_vec3(gl, shader_program, "cameraPosition", camera.view_obj.position.as_ptr());
        
        
        uniform_vec4(gl, shader_program, "mtl.ambient", object.material.ambient.as_ptr());
        uniform_vec4(gl, shader_program, "mtl.diffuse", object.material.diffuse.as_ptr());
        uniform_vec4(gl, shader_program, "mtl.specular", object.material.specular.as_ptr());
        uniform_f32(gl, shader_program, "mtl.shininess", object.material.shininess);
        
        for i in 0..server.lights.len() {
            let light = &server.lights[i];
            uniform_vec3(gl, shader_program, format!("lights[{}].position",i).as_str(), light.camera.view_obj.position.as_ptr());
            uniform_vec3(gl, shader_program, format!("lights[{}].direction",i).as_str(), light.camera.view_obj.forward.as_ptr());
            uniform_vec4(gl, shader_program, format!("lights[{}].color",i).as_str(), light.color.as_ptr());
            uniform_f32(gl, shader_program, format!("lights[{}].strength",i).as_str(), light.strength);

        }
        generate_vertex_array(gl);
        generate_buffers(gl, vertices, indices);
        
        generate_texture_data(gl, texture_data);
        
        load_vertex_attributes(gl);


        gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        gl.DrawElements(GL_TRIANGLES, (SIZE_OF_F32*indices.len()).try_into().unwrap(), GL_UNSIGNED_INT, 0 as *const _);
        
        gl.BindVertexArray(0);

        gl.BindBuffer(GL_ARRAY_BUFFER, 0);
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        
        delete_shader_program(gl, shader_program);

    }
}   
