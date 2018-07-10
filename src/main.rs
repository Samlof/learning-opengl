#![allow(dead_code)]
#![allow(unused_variables)]

extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate cgmath;

mod render_gl;
mod camera;
mod model;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Vector3};

fn main() {
    let mut screen_width = 900;
    let mut screen_height = 700;

    let start_time = std::time::SystemTime::now();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    sdl.mouse().set_relative_mouse_mode(true);

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", screen_width, screen_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, screen_width as gl::types::GLint, screen_height as gl::types::GLint);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }
    let shader_program = render_gl::Program::from_shaders(
        include_str!("triangle.vert"),
        include_str!("triangle.frag")
    ).unwrap();



    let mut nr_attribs: gl::types::GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attribs);
    }
    println!("Max vertex attribs {}", nr_attribs);

    let model = model::Model::static_new();

    let tex_coords = [
        0.0f32, 0.0,  // lower-left corner  
        1.0, 0.0,  // lower-right corner
        0.5, 1.0   // top-center corner
    ];


    // Texture loading
    let texture1 = create_texture("container.jpg");
    let texture2 = create_texture("awesomeface.png");
    // Set textures
    shader_program.set_used();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
    }
    
    // Cube positions
    let cube_positions = [
        Vector3{x: 0.0f32,y:  0.0f32,z:  0.0f32}, 
        Vector3{x: 2.0f32,y:  5.0f32,z: -15.0f32}, 
        Vector3{x:-1.5f32,y: -2.2f32,z: -2.5f32},  
        Vector3{x:-3.8f32,y: -2.0f32,z: -12.3f32},  
        Vector3{x: 2.4f32,y: -0.4f32,z: -3.5f32},  
        Vector3{x:-1.7f32,y:  3.0f32,z: -7.5f32},  
        Vector3{x: 1.3f32,y: -2.0f32,z: -2.5f32},  
        Vector3{x: 1.5f32,y:  2.0f32,z: -2.5f32}, 
        Vector3{x: 1.5f32,y:  0.2f32,z: -1.5f32}, 
        Vector3{x:-1.3f32,y:  1.0f32,z: -1.5f32}  
    ];

    // Camera variables
    let camera_pos = Vector3{x: 0.0f32, y: 0.0, z: 3.0};
    let world_up = Vector3{x: 0.0f32, y: 1.0, z: 0.0};

    let yaw = 270.0f32; // For some reason have to put in a lot of yaw at beginning
    let pitch = 0f32;

    let mut camera = camera::Camera::new(
        camera_pos, world_up, yaw, pitch);

    // Time variables
    let mut delta_time = 0f32;
    let mut last_frame = 0f32;

    let mut first_mouse = true;

    println!("Starting main!");
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                // Resize event
                sdl2::event::Event::Window {
                    timestamp,
                    window_id,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized(width, height) => unsafe {
                        // Update projection and gl viewport
                        screen_width = width as u32;
                        screen_height = height as u32;
                        gl::Viewport(0, 0, width, height);
                    },
                    _ => {}
                }, // Window events end
                sdl2::event::Event::KeyDown{keycode: Some(keycode), ..} => {
                    if keycode == sdl2::keyboard::Keycode::Escape {
                        break 'main;
                    }
                },
                sdl2::event::Event::MouseMotion{xrel, yrel, ..} => {
                    if first_mouse { first_mouse = false; break; }
                    let yrel = -yrel;
                    camera.process_mouse_movement(xrel as f32, yrel as f32);
                },
                sdl2::event::Event::MouseWheel{y, ..} => {
                    camera.process_mouse_scroll(y as f32);
                },
                _ => {}
            }
        }

        use camera::CameraMovement;
        // Check whether a key is down
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            camera.process_keyboard(CameraMovement::FORWARD, delta_time)
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            camera.process_keyboard(CameraMovement::BACKWARD, delta_time)
        }
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            camera.process_keyboard(CameraMovement::LEFT, delta_time)
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            camera.process_keyboard(CameraMovement::RIGHT, delta_time)
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Calculate time
        let current_frame = std::time::SystemTime::now().duration_since(start_time).unwrap();
        let current_frame = duration_into_float(current_frame);
        let time = current_frame;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // Projection matrix
        let projection : Matrix4<f32> = cgmath::PerspectiveFov{
            fovy: Deg(camera.get_zoom()).into(),
            aspect: screen_width as f32 / screen_height as f32,
            near: 0.1,
            far: 100.0
        }.into();        

        shader_program.set_mat4("view", camera.get_view_matrix().as_ptr());
        shader_program.set_mat4("projection", projection.as_ptr());
        
        shader_program.set_used();
        unsafe {

            gl::BindVertexArray(model.get_vao());

            for i in 0..10 {
                let mut model = Matrix4::from_translation(cube_positions[i]);
                model = model * Matrix4::from_axis_angle(
                    cgmath::Vector3{x: 1.0, y: 0.3, z: 0.5}.normalize(),
                    Deg(10.0 * (i+1) as f32 * time)
                );
                shader_program.set_mat4("model", model.as_ptr());

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            //gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}

fn duration_into_float(duration: std::time::Duration) -> f32 {
        let secs1 = duration.as_secs();
        let secs = secs1 as f32;
        let ms = duration.subsec_millis() as f32 / 1000.0;
        let time = secs + ms;
        return time;
}

fn create_texture(name: &str) -> gl::types::GLuint {
    let mut texture = 0;
    unsafe {
        stb_image::stbi_set_flip_vertically_on_load(true as i32); 
        // gen texture binding
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);

        // Load the texture into memory
        let (mut width, mut height, mut nr_channels) = (0, 0, 0);
        let data = stb_image::stbi_load(
            std::ffi::CString::new(format!("Resources/{}", name)).unwrap().as_bytes_with_nul().as_ptr() as *const i8,
            &mut width, &mut height, &mut nr_channels, 0
        );
        
        

        if width == 0 && height == 0 {
            panic!("{} wasn't loaded properly!", name);
        }

        // Push texture to gpu
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as gl::types::GLint,
            width, height,
            0,
            if name.ends_with("jpg") {gl::RGB} else {gl::RGBA},
            gl::UNSIGNED_BYTE,
            data as *const gl::types::GLvoid
        );
        //gl::GenerateMipmap(gl::TEXTURE_2D);

        // Free texture from memory
        stb_image::stbi_image_free(data as *mut std::os::raw::c_void);
    }
    return texture;
}