use gl;
use std;


pub struct Model {
    vao : gl::types::GLuint,
}

impl Model{
    pub fn new(vertices : Vec<f32>, indices : Vec<gl::types::GLuint>) -> Self {
        let vbo = create_triangle_vbo(vertices);
        let ebo = create_square_ebo(indices);
        Model {
            vao: create_triangle_vao(vbo, ebo)
        }
    }
    pub fn cube() -> Self {
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 3, 1, 2, 3];
        Self::new(get_cube_vertices(), indices)
    }

    
    pub fn light() -> Self {
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 3, 1, 2, 3];
        let vbo = create_triangle_vbo(get_cube_vertices());
        let ebo = create_square_ebo(indices);
        Model {
            vao: create_triangle_vao(vbo, ebo)
        }
    }

    pub fn get_vao(&self) -> gl::types::GLuint { self.vao }
}


fn create_triangle_vbo(vertices: Vec<f32>) -> gl::types::GLuint {
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

fn create_square_ebo(indices: Vec<gl::types::GLuint>) -> gl::types::GLuint {
    use std::mem::size_of;

    let mut ebo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Copy index array to gl
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
    }

    return ebo;
}

fn create_triangle_vao(vbo: gl::types::GLuint, ebo: gl::types::GLuint) -> gl::types::GLuint {
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Copy vertice array to gl
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Set Position vertex attrib pointers
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // Tex
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }
    return vao;
}

fn create_light_vao(vbo: gl::types::GLuint, ebo: gl::types::GLuint) -> gl::types::GLuint {
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Copy vertice array to gl
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Set Position vertex attrib pointers
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }
    return vao;
}

fn get_cube_vertices() -> Vec<f32> {
        vec![
            -0.5f32, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ]
}