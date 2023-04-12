use gl33::*;
use glm::{*};
use crate::rendering::draw::{draw_object, bind_texture_data};
use crate::scene::*;
use crate::rendering::{*, uniform::*};
use crate::ServerEngine;
use crate::types::*;
use sdl2::image::*;
use std::fs;

pub struct Client{
    pub camera : Camera,
    pub server : ServerEngine,
    pub gl : GlFns
}
pub fn rot_align(vec1 : Vec3, vec2 : Vec3) -> Matrix3<f32> {
    let axis = vec1.cross(&vec2);

    let cos_a = vec1.dot(&vec2);
    let k = 1.0 / (1.0 + cos_a);

    return Matrix3::new( (axis.x * axis.x * k) + cos_a,
                 (axis.y * axis.x * k) - axis.z, 
                 (axis.z * axis.x * k) + axis.y,
                 (axis.x * axis.y * k) + axis.z,  
                 (axis.y * axis.y * k) + cos_a,      
                 (axis.z * axis.y * k) - axis.x,
                 (axis.x * axis.z * k) - axis.y,
                 (axis.y * axis.z * k) + axis.x,  
                 (axis.z * axis.z * k) + cos_a,
                 )

}

impl Client {
    pub unsafe fn init_gl(&self) {
        let gl = &self.gl;
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_LEQUAL);



        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    pub unsafe fn load_all_obj_textures(&mut self) {
        for i in 0..self.server.objects.len() {
            self.server.objects[i].texture.gen_id(&self.gl);
        }
    }
    pub fn update_camera(&mut self, step : f32) {
        self.camera.view_obj.update_cam(step);
    }
    pub unsafe fn draw_object(&self, object : &Model, shader_program : u32, load_uniforms_fn : &dyn Fn(&GlFns, u32, &Client, &Model)) {
        draw::draw_object(self, object, shader_program, load_uniforms_fn)
    }
    pub unsafe fn draw_scene(&self, shader_program : u32) {
        self.gl.CullFace(GL_BACK);
        for object in &self.server.objects{
            let uniform_fn = |gl : &GlFns, shader_program : u32, client : &Client, model : &Model| {
                gl.DepthFunc(GL_LEQUAL);
                let camera = &client.camera;
                let mvp_obj = object.view_obj.look_at_up(object.view_obj.up).try_inverse().unwrap();
                let look_at = camera.view_obj.look_at();
                let projection = camera.projection;

                gl.ActiveTexture(GL_TEXTURE0);
                gl.BindTexture(GL_TEXTURE_2D, 0);
                
                gl.TexImage2D(
                    GL_TEXTURE_2D, 
                    0, 
                    GL_RGB.0.try_into().unwrap(), 
                    object.texture.data.width() as i32, 
                    object.texture.data.height() as i32, 
                    0, 
                    GL_RGB,
                    GL_UNSIGNED_BYTE, 
                    object.texture.data.without_lock().unwrap().as_ptr() as *const _
                );
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0.try_into().unwrap());
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0.try_into().unwrap());
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0.try_into().unwrap());
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0.try_into().unwrap());
                gl.GenerateMipmap(GL_TEXTURE_2D);

                gl.Uniform1ui(gl.GetUniformLocation(shader_program, format!("{}\0","ourTexture").as_ptr()), 0);
                
                uniform_matrix4(gl, shader_program, "mvp", mvp_obj.as_ptr());
                uniform_matrix4(gl, shader_program, "lookAt", look_at.as_ptr());
                uniform_matrix4(gl, shader_program, "projection", projection.as_ptr());
                
                
                uniform_vec3(gl, shader_program, "cameraDirection", camera.view_obj.forward.as_ptr());
                uniform_vec3(gl, shader_program, "cameraPosition", camera.view_obj.position.as_ptr());
                
                
                uniform_vec4(gl, shader_program, "mtl.ambient", model.material.ambient.as_ptr());
                uniform_vec4(gl, shader_program, "mtl.diffuse", model.material.diffuse.as_ptr());
                uniform_vec4(gl, shader_program, "mtl.specular", model.material.specular.as_ptr());
                uniform_f32(gl, shader_program, "mtl.shininess", model.material.shininess);

                for i in 0..client.server.lights.len() {
                    let light = &client.server.lights[i];
                    uniform_vec3(gl, shader_program, format!("lights[{}].position",i).as_str(), light.camera.view_obj.position.as_ptr());
                    uniform_vec3(gl, shader_program, format!("lights[{}].direction",i).as_str(), light.camera.view_obj.forward.as_ptr());
                    uniform_vec4(gl, shader_program, format!("lights[{}].color",i).as_str(), light.color.as_ptr());
                    uniform_matrix4(gl, shader_program, format!("lights[{}].lookAt",i).as_str(), light.camera.view_obj.look_at().as_ptr());
                    uniform_matrix4(gl, shader_program, format!("lights[{}].projection",i).as_str(), light.camera.projection.as_ptr()); 
                    uniform_f32(gl, shader_program, format!("lights[{}].strength",i).as_str(), light.strength);
                    
                    gl.ActiveTexture(gl33::GLenum(TryInto::<u32>::try_into(GL_TEXTURE1.0).unwrap()));
                    gl.BindTexture(GL_TEXTURE_2D_ARRAY, light.depth_map);
                    gl.TexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MAG_FILTER, GL_NEAREST.0.try_into().unwrap());
                    gl.TexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_NEAREST.0.try_into().unwrap());
                    gl.TexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE.0.try_into().unwrap());
                    gl.TexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE.0.try_into().unwrap());
                    gl.GenerateMipmap(GL_TEXTURE_2D_ARRAY);
                    gl.Uniform1i(gl.GetUniformLocation(shader_program, format!("{}\0","depthMaps").as_ptr()), light.depth_map.try_into().unwrap());
                }
                //gl.PolygonMode(GL_FRONT_AND_BACK, GL_TRIANGLES);
                
            };
            draw_object(self, object, shader_program, &uniform_fn);
        }
    }
    pub unsafe fn update_lights_buffers(&mut self, step : f32) {
        
        let gl = &self.gl;
        gl.DepthFunc(GL_LESS);
        gl.CullFace(GL_BACK);
        //gl.PolygonMode(GL_BACK, GL_TRIANGLES);

        for i in 0..self.server.lights.len() {
            self.server.lights[i].camera.view_obj.update_object(step);
            let mut depth_map_fbo = 0;
            
            gl.GenFramebuffers(1, &mut depth_map_fbo);
            if self.server.lights[i].depth_map == 0 {
                gl.GenTextures(1, &mut self.server.lights[i].depth_map);
            }
            gl.BindTexture(GL_TEXTURE_2D, self.server.lights[i].depth_map);    
            gl.TexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_DEPTH_COMPONENT.0.try_into().unwrap(), 
                light::LIGHT_RESOLUTION.0, 
                light::LIGHT_RESOLUTION.1, 
                0, 
                GL_DEPTH_COMPONENT, 
                GL_FLOAT, 
                0 as *const _
            );
            gl.GenerateMipmap(GL_TEXTURE_2D);

            gl.BindFramebuffer(GL_FRAMEBUFFER, depth_map_fbo);

            gl.FramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, self.server.lights[i].depth_map, 0);
            gl.Viewport(0, 0, light::LIGHT_RESOLUTION.0, light::LIGHT_RESOLUTION.1);
            gl.BindFramebuffer(GL_FRAMEBUFFER, depth_map_fbo);
            //gl.DrawBuffer(GL_NONE);

            gl.Clear(GL_DEPTH_BUFFER_BIT);
            let vert_shader = fs::read_to_string("assets/shaders/vertex_shaders/depth_vert.glsl").unwrap();
            let frag_shader = fs::read_to_string("assets/shaders/fragment_shaders/depth_frag.glsl").unwrap();
            
            let uniform_fn = |gl : &GlFns, shader_program : u32, client : &Client, model : &Model| {
                let mvp_obj = model.view_obj.look_at_up(model.view_obj.up).try_inverse().unwrap();
    
                uniform_matrix4(gl, shader_program, "mvp", mvp_obj.as_ptr());
                uniform_matrix4(gl, shader_program, "lightProjection", self.server.lights[i].camera.projection.as_ptr());
                uniform_matrix4(gl, shader_program, "lightLookAt", self.server.lights[i].camera.view_obj.look_at().as_ptr());

            };
    
            for object in &self.server.objects{
                self.draw_object(&object, draw::shader_program(gl, &vert_shader, &frag_shader), &uniform_fn);
            }
            gl.BindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
}