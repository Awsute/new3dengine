use gl33::*;
use glm::{*};
use crate::scene::*;
use crate::ServerEngine;
pub struct Client{
    pub camera : Camera,
    pub server : ServerEngine,
    pub gl : GlFns
}

impl Client{
    pub fn init_gl(&self){
        unsafe {
            self.gl.Enable(GL_DEPTH_TEST);
            self.gl.DepthFunc(GL_LEQUAL);
    
    
            self.gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0.try_into().unwrap());
            self.gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0.try_into().unwrap());
            self.gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0.try_into().unwrap());
            self.gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0.try_into().unwrap());
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
        }
    }
    pub fn update_scene(&self, timestep : f32){

    }
    pub fn draw_scene(&self){

    }
}