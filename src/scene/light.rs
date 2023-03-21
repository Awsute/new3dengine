use glm::*;
use gl33::*;
use std::fs;
use crate::rendering::{*, draw::draw_object};
use crate::{camera::*, object::*, engine::*};

pub struct Light{
    pub camera : Camera,
    pub color : Vector4<f32>,
    pub strength : f32,
    pub shadow_resolution : (i32, i32)
}

impl Light {
    pub unsafe fn gen_depth_buffer(&self, client : &Client) {
        let gl = &client.gl;
        for object in &client.server.objects{
            let mut depthMapFBO = 0;

            gl.GenFramebuffers(1, &mut depthMapFBO);

            let mut depthMap = 0;

            gl.GenTextures(1, &mut depthMap);
            gl.BindTexture(GL_TEXTURE_2D, depthMap);
            gl.TexImage2D(
                GL_TEXTURE_2D, 
                0,
                GL_DEPTH_COMPONENT.0.try_into().unwrap(), 
                self.shadow_resolution.0, 
                self.shadow_resolution.1, 
                0, 
                GL_DEPTH_COMPONENT, 
                GL_FLOAT, 
                0 as *const _
            );
            gl.BindFramebuffer(GL_FRAMEBUFFER, depthMapFBO);
            gl.FramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, depthMap, 0);
            gl.Viewport(0, 0, self.shadow_resolution.0, self.shadow_resolution.1);
            gl.BindFramebuffer(GL_FRAMEBUFFER, depthMapFBO);



            gl.Clear(GL_DEPTH_BUFFER_BIT);
            
            draw_object(client, &object, &fs::read_to_string("assets/shaders/vertex_shaders/depth_vert.glsl").unwrap(), &fs::read_to_string("assets/shaders/fragment_shaders/depth_frag.glsl").unwrap());

            gl.BindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
}