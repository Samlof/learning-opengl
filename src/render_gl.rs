use gl;
use std;
use std::ffi::CString;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(vertex_source: &str, frag_source: &str) -> Result<Program, String> {
        let v_shader = Shader::from_vert_source(vertex_source).unwrap();
        let f_shader = Shader::from_frag_source(frag_source).unwrap();
        let shaders = [v_shader, f_shader];

        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders.iter() {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }
        unsafe {
            gl::LinkProgram(program_id);
        }

        // error handling
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(format!("Error linking program: {}", error.to_string_lossy().into_owned()));
        }

        for shader in shaders.iter() {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const gl::types::GLchar);
            if uniform_loc == -1 {
                println!("Couldn't find uniform location: \"{}\"!", name);
            }
            self.set_used();
            gl::Uniform1i(uniform_loc, value as gl::types::GLint);
        }
    }
    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const gl::types::GLchar);
            if uniform_loc == -1 {
                println!("Couldn't find uniform location: \"{}\"!", name);
            }
            self.set_used();
            gl::Uniform1i(uniform_loc, value as gl::types::GLint);
        }
    }
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const gl::types::GLchar);
            if uniform_loc == -1 {
                println!("Couldn't find uniform location: \"{}\"!", name);
            }
            self.set_used();
            gl::Uniform1f(uniform_loc, value as gl::types::GLfloat);
        }
    }
    pub fn set_mat4(&self, name: &str, value: *const gl::types::GLfloat) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const gl::types::GLchar);
            if uniform_loc == -1 {
                println!("Couldn't find uniform location: \"{}\"!", name);
            }
            self.set_used();
            gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, value);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &str, kind: gl::types::GLuint) -> Result<Shader, String> {
        shader_from_source(source, kind).map(|i| Shader { id: i })
    }
    pub fn from_vert_source(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let source = CString::new(source).map_err(|_| "Source included NUL character")?;
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    // Check for compile error
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(format!("Shader error: {}", error.to_string_lossy()));
    }
    Ok(id)
}

fn create_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    unsafe {
        buffer.set_len(len as usize + 1);
    }
    unsafe { CString::from_vec_unchecked(buffer) }
}
