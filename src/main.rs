extern crate sdl2;
extern crate gl33;
extern crate nalgebra as glm;

use gl33::{*, gl_core_types::*, gl_enumerations::*, gl_groups::*, global_loader::*};
use sdl2::pixels::Color;
use sdl2::image::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::SDL_GL_GetProcAddress;
use std::time::Duration;
use glm::*;
//mod mesh_loader;

mod scene;
mod rendering;
mod types;
mod engine;

use engine::*;
use types::*;
use crate::rendering::{draw::*};

use scene::*;
//use crate::mesh_loader::*;


const VERT_SHADER: &str = r#"#version 460

  
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;
    layout (location = 3) uniform mat4 mvp;

    //out vec3 ourNormal;
    out vec2 texCoord;

    void main() {
        gl_Position = mvp*vec4(aPos.x,aPos.y,-aPos.z, 1.0);
        //ourNormal = aNormal;
        texCoord = aTexCoord;

    }
"#;

const FRAG_SHADER: &str = r#"#version 460
    //in vec3 ourNormal;
    in vec2 texCoord;


    out vec4 FragColor;

    uniform sampler2D ourTexture;
    void main() {
        FragColor = vec4(1.0,1.0,1.0,1.0);
        //FragColor = texture(ourTexture, texCoord);
    }
"#;


pub fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .opengl()
        //.fullscreen()
        .position_centered()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    
    
    let gl = unsafe { GlFns::load_from(&|p| SDL_GL_GetProcAddress(p as *const i8) as _).unwrap() };
    let server = ServerEngine{
        objects : Vec::new(),
        lights : Vec::new()
    };
    let mut current_camera = Camera{
        position : Vec3::new(0.0,0.0,0.0),
        direction : Vec3::new(0.0,0.0,1.0),
        projection : Mat4::new_perspective(window.size().0 as f32/window.size().1 as f32, 90_f32.to_radians(), 0.1, 100.0),
        velocity : Vec3::new(0.0,0.0,0.0),
        rotational_velocity : Vec3::new(0.0,0.0,0.0)
    };
    let mut current_client = Client{
        camera : current_camera,
        server : server,
        gl : gl
    };
    unsafe{

        current_client.init_gl();
    }
    let object = Model { 
        mesh: Mesh::load_obj_file("assets/objects/normalized_cube.obj".to_string()), 
        material: Material { ambient: Vec4::new(0.0,0.0,0.0,1.0), diffuse: Vec4::new(0.0,0.0,0.0,1.0), specular: Vec4::new(0.0,0.0,0.0,1.0), shininess: 0.0 }, 
        texture: "assets/textures/travisScot.png", 
        velocity: Vec3::new(0.0,0.0,0.0), 
        rotational_velocity: Vec3::new(0.0,0.0,0.0) 
    };
    current_client.server.objects.push(object);
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        unsafe{
            current_client.draw_scene(FRAG_SHADER, VERT_SHADER);
        }
        // The rest of the game loop goes here...
        
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
