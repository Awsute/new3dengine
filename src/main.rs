extern crate sdl2;
extern crate gl33;
extern crate nalgebra as glm;

use gl33::{*, gl_core_types::*, gl_enumerations::*, gl_groups::*, global_loader::*};
use rendering::draw;
use sdl2::pixels::Color;
use sdl2::image::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::SDL_GL_GetProcAddress;
use std::time::Duration;
use glm::*;
use std::fs;
//mod mesh_loader;

mod scene;
mod rendering;
mod types;
mod engine;

use engine::*;

use types::*;
use crate::rendering::{draw::*,uniform::*};

use scene::*;
//use crate::mesh_loader::*;


pub fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screen_size = video_subsystem.display_bounds(0).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo",screen_size.width(),screen_size.height()-50)
        .opengl()
        .fullscreen()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let max_fps = 60.0;
    
    let gl = unsafe { GlFns::load_from(&|p| SDL_GL_GetProcAddress(p as *const i8) as _).unwrap() };
    let server = ServerEngine{
        objects : Vec::new(),
        lights : Vec::new()
    };
    let current_camera = Camera::new(
        Vec3::new(10.0,0.0,0.0),
        Vec3::new(-1.0,0.0,0.0).normalize(),

        Mat4::new_perspective(window.size().0 as f32/window.size().1 as f32, 60_f32.to_radians(), 0.1, 100.0)
    );
    let mut current_client = Client{
        camera : current_camera,
        server : server,
        gl : gl
    };
    

    let vert_shader = fs::read_to_string("assets/shaders/vertex_shaders/phong_vert.glsl").unwrap();
    let frag_shader = fs::read_to_string("assets/shaders/fragment_shaders/phong_frag.glsl").unwrap();

    unsafe{
        current_client.init_gl();
    }
    let mut object = Model {
        mesh: Mesh::load_obj_file("assets/objects/normalized_teapot.obj".to_string()), 
        material: Material::new(
            Vec4::new(0.1,0.1,0.1,1.0), 
            Vec4::new(0.9,0.9,0.9,1.0),
            Vec4::new(1.0,1.0,1.0,1.0),
            16.0 
        ),
        texture: Texture::gen_new_texture("assets/textures/travisScot.png"),
        view_obj : ViewObject::empty()
    };


    let mut object0 = Model {
        mesh: Mesh::load_obj_file("assets/objects/bunny.obj".to_string()), 
        material: Material::new(
            Vec4::new(0.1,0.1,0.1,1.0), 
            Vec4::new(0.0,0.5,0.5,1.0),
            Vec4::new(1.0,0.9,0.9,1.0),
            16.0 
        ),
        texture: Texture::gen_new_texture("assets/textures/white.png"),
        view_obj : ViewObject::empty()
    };

    let mut object1 = Model {
        mesh: Mesh::load_obj_file("assets/objects/normalized_cube.obj".to_string()), 
        material: Material::new(
            Vec4::new(0.1,0.1,0.1,1.0), 
            Vec4::new(0.9,0.9,0.9,1.0),
            Vec4::new(1.0,1.0,1.0,1.0),
            64.0 
        ),
        texture: Texture::gen_new_texture("assets/textures/white.png"),
        view_obj : ViewObject::from_position(Vec3::new(5.0,1.0,1.0))
    };



    let light = Light{
        camera : Camera::new(
            Vec3::new(30.0,0.0,0.0), 
            Vec3::new(-1.0,0.0,0.0).normalize(), 
            //Mat4::new_orthographic(-50.0, 50.0, -50.0, 50.0, 0.01, 100.0)
            Mat4::new_perspective(1.0, 90_f32.to_radians(), 0.1, 100.0)
            ,
        ),
        color : Vec4::new(0.0,1.0,0.0,1.0),
        strength : 1.0,
        depth_map : 0,
    };
    let light0 = Light{
        camera : Camera::new(
            Vec3::new(10.0,-10.0,0.0), 
            Vec3::new(-1.0,1.0,0.0).normalize(), 
            //Mat4::new_orthographic(-50.0, 50.0, -50.0, 50.0, 0.01, 100.0)
            Mat4::new_perspective(1.0, 90_f32.to_radians(), 0.1, 100.0)
            ,
        ),
        color : Vec4::new(1.0,0.0,1.0,1.0),
        strength : 0.0,
        depth_map : 0,
    };

    let light1 = Light{
        camera : Camera::new(
            Vec3::new(-30.0,0.0,0.0), 
            Vec3::new(1.0,0.0,0.0).normalize(), 
            //Mat4::new_orthographic(-50.0, 50.0, -50.0, 50.0, 0.01, 100.0)
            Mat4::new_perspective(1.0, 90_f32.to_radians(), 0.1, 100.0)
            ,
        ),
        color : Vec4::new(1.0,1.0,1.0,1.0),
        strength : 1.0,
        depth_map : 0,
    };
    current_client.server.objects.push(object);
    current_client.server.objects.push(object0);
    current_client.server.objects.push(object1);

    current_client.server.lights.push(light);
    current_client.server.lights.push(light0);
    current_client.server.lights.push(light1);


    current_client.server.objects[0].view_obj.velocity = Vec3::new(0.0,0.0,0.0);
    current_client.server.objects[0].view_obj.rotational_velocity = Vec3::new(0.0,1.0,0.0);
    current_client.server.objects[1].view_obj.position = Vec3::new(3.0,3.0,0.0);
    current_client.server.objects[1].view_obj.rotational_velocity = Vec3::new(0.5,1.0,0.0);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = fps::start_timer();
    'running: loop {
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    current_client.camera.view_obj.velocity.z = 10.0
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    current_client.camera.view_obj.velocity.z = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    current_client.camera.view_obj.velocity.z = -10.0
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    current_client.camera.view_obj.velocity.z = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    current_client.camera.view_obj.velocity.x = 10.0
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    current_client.camera.view_obj.velocity.x = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    current_client.camera.view_obj.velocity.x = -10.0
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    current_client.camera.view_obj.velocity.x = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    current_client.camera.view_obj.velocity.y = -10.0
                },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                    current_client.camera.view_obj.velocity.y = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    current_client.camera.view_obj.velocity.y = 10.0
                },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                    current_client.camera.view_obj.velocity.y = 0.0
                },

                
                
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    current_client.camera.view_obj.rotational_velocity.y = 1.0
                },

                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    current_client.camera.view_obj.rotational_velocity.y = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    current_client.camera.view_obj.rotational_velocity.y = -1.0
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    current_client.camera.view_obj.rotational_velocity.y = 0.0
                },



                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    current_client.camera.view_obj.rotational_velocity.x = 1.0
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    current_client.camera.view_obj.rotational_velocity.x = 0.0
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    current_client.camera.view_obj.rotational_velocity.x = -1.0
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    current_client.camera.view_obj.rotational_velocity.x = 0.0
                },
                _ => {}
            }
        }

        let mut frame_time = fps::end_timer(timer);
        if max_fps < 1.0/frame_time {
            frame_time = 1.0/max_fps;
        }
        else {
            println!("{}", 1.0/frame_time);
        }
        current_client.server.update_scene(frame_time);
        current_client.update_camera(frame_time);
        unsafe {
            current_client.update_lights_buffers(frame_time);

            current_client.gl.Viewport(0, 0, window.size().0 as i32, window.size().1 as i32);
            
            current_client.gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            current_client.draw_scene(draw::shader_program(&current_client.gl, &vert_shader, &frag_shader));

        }

        // The rest of the game loop goes here...
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / max_fps as u32));
        
        timer = fps::start_timer();
    }
}
