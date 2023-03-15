use gl33::*;
use glm::{*};
use crate::rendering::draw::draw_object;
use crate::scene::*;
use crate::rendering::*;
use crate::ServerEngine;
use crate::types::*;
use sdl2::image::*;

pub struct Client{
    pub camera : Camera,
    pub server : ServerEngine,
    pub gl : GlFns
}
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

impl Client {
    pub unsafe fn init_gl(&self) {
        let gl = &self.gl;
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_LEQUAL);


        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0.try_into().unwrap());
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    pub fn update_camera(&mut self, step : f32) {
        let point_at = self.camera.look_at().try_inverse().unwrap();
        let vo = &mut self.camera.view_obj;
        let new_rot = Mat4::new_rotation(vo.rotational_velocity*step) * vo.direction.insert_row(3, 0.0);
        vo.direction = new_rot.remove_row(3);
        
        let transformed_velocity = point_at*vo.velocity.insert_row(3,0.0);
        vo.position = vo.position - transformed_velocity.remove_row(3)*step;
    }

    pub unsafe fn draw_scene(&self, frag_shader : &str, vert_shader : &str) {
        let gl = &self.gl;
        for object in &self.server.objects{
            draw_object(gl, &self.camera, object, vert_shader, frag_shader)
        }
    }
    
}