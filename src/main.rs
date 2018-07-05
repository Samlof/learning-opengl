#![allow(dead_code)]
#![allow(unused_variables)]

extern crate gl;
extern crate sdl2;
extern crate stb_image;

pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }
    let shader_program = render_gl::Program::from_shaders(
        include_str!("triangle.vert"),
        include_str!("triangle.frag")
    ).unwrap();

    let vertices = [
        // positions         // colors
        0.5f32, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
    ];
    let indices: [gl::types::GLuint; 6] = [0, 1, 3, 1, 2, 3];

    let mut nr_attribs: gl::types::GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attribs);
    }
    println!("Max vertex attribs {}", nr_attribs);

    let vbo1: gl::types::GLuint = create_triangle_vbo(vertices);
    let vao1: gl::types::GLuint = create_triangle_vao(vbo1);

    let tex_coords = [
    0.0f32, 0.0,  // lower-left corner  
    1.0, 0.0,  // lower-right corner
    0.5, 1.0   // top-center corner
    ];

    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as gl::types::GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);

        let (mut width, mut height, mut nr_channels) = (0, 0, 0);
        let data = stb_image::stbi_load(
            std::ffi::CString::new("Resources/container.jpg").unwrap().as_bytes_with_nul().as_ptr() as *const i8,
            &mut width, &mut height, &mut nr_channels, 0
        );
        println!("{} {} {}", width, height, nr_channels);
    }


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
                        gl::Viewport(0, 0, width, height);
                    },
                    _ => {}
                }, // Window events end
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Calculate time to use for green value
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs1 = time.as_secs() % 1000;
        let secs = secs1 as f32;
        let ms = time.subsec_millis() as f32 / 1000.0;
        let time = secs + ms;
        let green_value = (time.sin() / 2.0) + 0.5;

        //shader_program.set_uniform_4f("myColor", green_value);
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao1);
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}

fn create_triangle_vbo(vertices: [f32; 18]) -> gl::types::GLuint {
    use std::mem::size_of;

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid,                // pointer to data
            gl::STATIC_DRAW,                                              // usage
        );
    }

    return vbo;
}

fn create_triangle_vao(vbo: gl::types::GLuint) -> gl::types::GLuint {
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Copy vertice array to gl
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        /*
        // Copy index array to gl
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        */

        // Set Position vertex attrib pointers
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // Color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }
    return vao;
}
