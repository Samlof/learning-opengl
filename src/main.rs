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
    let lightning_shader = render_gl::Program::from_shaders(
        include_str!("triangle.vert"),
        include_str!("light.frag")
    ).unwrap();
    let lamp_shader = render_gl::Program::from_shaders(
        include_str!("lamp.vert"),
        include_str!("lamp.frag")
    ).unwrap();

    let mut nr_attribs: gl::types::GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attribs);
    }
    println!("Max vertex attribs {}", nr_attribs);

    let cube = model::Model::cube();
    let light_vao = model::Model::light();

    // Light location
    let light_pos = Vector3{x: 1.2f32, y: 1.0, z: 2.0};

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

    
    lightning_shader.set_used();
    lightning_shader.set_vec3("objectColor", 1.0, 0.5, 0.31);
    lightning_shader.set_vec3("lightColor", 1.0, 1.0, 1.0);
    lightning_shader.set_vec3("lightPos", light_pos.x, light_pos.y, light_pos.z);

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
        {
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
        let model = Matrix4::from_translation((0.0, 0.0, 0.0).into());
        // Draw cube
        lightning_shader.set_used();
        lightning_shader.set_mat4("view", camera.get_view_matrix().as_ptr());
        lightning_shader.set_mat4("projection", projection.as_ptr());
        lightning_shader.set_mat4("model", model.as_ptr());

        unsafe {
            gl::BindVertexArray(cube.get_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Draw lamp
        let mut model = Matrix4::from_translation(light_pos);
        model = model * Matrix4::from_scale(0.2);
        lamp_shader.set_used();
        lamp_shader.set_mat4("view", camera.get_view_matrix().as_ptr());
        lamp_shader.set_mat4("projection", projection.as_ptr());
        lamp_shader.set_mat4("model", model.as_ptr());

        unsafe {
            gl::BindVertexArray(light_vao.get_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
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