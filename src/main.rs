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
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aTexCoord;
    layout (location = 3) uniform mat4 transform;

    out vec3 ourColor;
    out vec2 texCoord;

    void main() {
        gl_Position = transform*vec4(aPos.x,aPos.y,-aPos.z, 1.0);
        ourColor = aColor;
        texCoord = aTexCoord;

    }
"#;

const FRAG_SHADER: &str = r#"#version 460
    in vec3 ourColor;
    in vec2 texCoord;


    out vec4 FragColor;

    uniform sampler2D ourTexture;
    void main() {
        FragColor = texture(ourTexture, texCoord) * vec4(ourColor, 1.0);
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
    
    unsafe {
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_GEQUAL);


        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR.0.try_into().unwrap());
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0.try_into().unwrap());
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }


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
            
            let texture = "assets/textures/travisScot.png";
            let vertices =
            vec![
                //position        //color          //texCoords
                0.5,  0.5, 1.0,   1.0, 1.0, 1.0,   1.0, 0.0,   // top right
                0.5, -0.5, 1.0,   1.0, 1.0, 1.0,   1.0, 1.0,   // bottom right
               -0.5, -0.5, 1.0,   1.0, 1.0, 1.0,   0.0, 1.0,   // bottom left
               -0.5,  0.5, 1.0,   1.0, 1.0, 1.0,   0.0, 0.0    // top left 
            ];
            let index_buffer = vec![    
                0, 1, 3, 
                1, 2, 3
            ];
            draw_object(&gl, vertices, index_buffer, VERT_SHADER, FRAG_SHADER, texture);
        }
        // The rest of the game loop goes here...
        
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
