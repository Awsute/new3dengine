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
pub unsafe fn unbind_vertex_array(gl : &GlFns) {
    gl.BindVertexArray(0);

}
pub unsafe fn bind_vertex_buffer(gl : &GlFns, vertices : &Vec<f32>) {
    
    let mut vbo = 0;
    gl.GenBuffers(1, &mut vbo);
    gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

    buffer_data(
        gl,
        GL_ARRAY_BUFFER, 
        bytemuck::cast_slice(vertices.as_slice()), 
        GL_STATIC_DRAW
    );


}
pub unsafe fn bind_element_buffer(gl : &GlFns, indices : &Vec<u32>) {
    let mut ebo = 0;

    gl.GenBuffers(1, &mut ebo);
    gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

    buffer_data(
        gl,
        GL_ELEMENT_ARRAY_BUFFER,
        bytemuck::cast_slice(indices.as_slice()), 
        GL_STATIC_DRAW
    );
}
pub unsafe fn unbind_vertex_buffer(gl : &GlFns) {
    gl.BindBuffer(GL_ARRAY_BUFFER, 0);
}

pub unsafe fn unbind_element_buffer(gl : &GlFns) {
    gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

}
pub unsafe fn bind_texture_data(gl : &GlFns, texture_data : u32, shader_program : u32) {
    gl.GenerateMipmap(GL_TEXTURE_2D);
    gl.Uniform1ui(gl.GetUniformLocation(shader_program, format!("{}\0","ourTexture").as_ptr()), texture_data);

}

pub fn draw_object(client : &Client, object : &Model, shader_program : u32, load_uniforms_fn : &dyn Fn(&GlFns, u32, &Client, &Model)) {
    unsafe {
        let gl = &client.gl;
        
        let indices = &object.mesh.index_buffer;
        let vertices = &object.mesh.vertex_buffer;

        load_uniforms_fn(gl, shader_program, client, object);

        generate_vertex_array(gl);
        bind_vertex_buffer(gl, vertices);
        bind_element_buffer(gl, indices);
                
        load_vertex_attributes(gl);

        gl.DrawElements(GL_TRIANGLES, (SIZE_OF_F32*indices.len()).try_into().unwrap(), GL_UNSIGNED_INT, 0 as *const _);
        
        unbind_element_buffer(gl);
        unbind_vertex_buffer(gl);
        unbind_vertex_array(gl);
        gl.BindTexture(GL_TEXTURE_2D, 0);
        delete_shader_program(gl, shader_program);

    }
}   
