extern crate gl33;
extern  crate bytemuck;
use std::{ops::Deref, str::from_utf8};

use sdl2::image::*;
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
pub fn draw_object(gl:&GlFns, object : Model, vert_shader : &str, frag_shader : &str, texture : &str){
    unsafe {
        let indices = object.mesh.index_buffer;
        let vertices = object.mesh.vertex_buffer;
        let texture_data = object.texture;
        let mut vao = 0;
        let mut ebo = 0;
        let mut tex = 0;
        
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);



        gl.GenBuffers(1, &mut ebo);
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

        buffer_data(
            gl,
            GL_ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(indices.as_slice()), 
            GL_STATIC_DRAW
        );


        
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


        let size_of_f32 = std::mem::size_of::<f32>();
        let stride = (8*size_of_f32).try_into().unwrap();
        
        //Position
        let mut vbo = 0;
        
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

        buffer_data(
            gl,
            GL_ARRAY_BUFFER, 
            bytemuck::cast_slice(vertices.as_slice()), 
            GL_STATIC_DRAW
        );

        //point
        gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            stride,
            0 as *const _,
        );
        gl.EnableVertexAttribArray(0);

        //normal
        gl.VertexAttribPointer(
            1,
            3,
            GL_FLOAT,
            0,
            stride,
            (3*size_of_f32) as *const _
        );
        gl.EnableVertexAttribArray(1);

        //tex
        gl.VertexAttribPointer(
            2,
            2,
            GL_FLOAT,
            0,
            stride,
            (6*size_of_f32) as *const _
        );
        gl.EnableVertexAttribArray(2);





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
        //gl.DrawArrays(GL_TRIANGLES, 0, 3);


        

        gl.PolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        let rotation = Matrix4::new_rotation(Vector3::new(0.0_f32,0.0,0.0));
        let transform = Matrix4::new_translation(&Vector3::new(0.0_f32, 1.0, -5.0));
        let projection = Matrix4::new_perspective(11.0/8.5, 90_f32.to_radians(),0.1, 100.0);
        gl.UniformMatrix4fv(
            gl.GetUniformLocation(shader_program, format!("{}\0", "mvp").as_ptr()), 
            1, 
            GL_FALSE.0.try_into().unwrap(), 
            (projection*rotation*transform*Matrix4::identity()).as_ptr()
        );

        gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        gl.DrawElements(GL_TRIANGLES, (std::mem::size_of_val(&indices)).try_into().unwrap(), GL_UNSIGNED_INT, 0 as *const _);
        gl.BindVertexArray(0);

        gl.BindBuffer(GL_ARRAY_BUFFER, 0);
        //gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        

        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

    }
}   
