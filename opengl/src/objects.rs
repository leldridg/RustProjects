use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};

use gl::types::{GLuint, GLint, GLchar, GLenum};

// an opengl shader
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        // create, source, and compile shader

        let id: u32 = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        // check if an error occurred while compiling

        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }

        if success == 0 { // an error occurred
            let mut len: GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); } // get length of error message

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

// a sequence of shaders calls
pub struct Program {
    id: GLuint,
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        // create, attach, and link program

        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        // check if an error occurred while linking
        
        let mut success: GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }

        if success == 0 { // an error occurred
            let mut len: GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); } // get length of error message

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Program { id })
    }

    pub fn set(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_program() -> Result<Program, &'static str> {
    let vert_shader = Shader::from_source(&CString::new(include_str!(".vert")).unwrap(), gl::VERTEX_SHADER).unwrap();
    let frag_shader = Shader::from_source(&CString::new(include_str!(".frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    Ok(shader_program)
}

// vertex buffer object
pub struct Vbo { 
    pub id: GLuint,
}

impl Vbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Vbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

// index buffer object
pub struct Ibo { 
    pub id: GLuint,
}

impl Ibo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Ibo { id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

// vertex array object
pub struct Vao { 
    pub id: GLuint,
}

impl Vao {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, 
                2, 
                gl::FLOAT, 
                gl::FALSE,
                (2 *std::mem::size_of::<f32>()) as GLint,
                null()
            );
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}