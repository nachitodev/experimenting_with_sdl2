use gl::types::*;
use gl::{CompileShader, CreateShader, DeleteShader, GetShaderInfoLog, GetShaderiv, ShaderSource};
use std::ffi::CString;

pub fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = CreateShader(shader_type);

        let c_str = CString::new(src).unwrap();

        ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());

        CompileShader(shader);

        let mut success = gl::FALSE as GLint;

        GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "Error al compilar el shader: {}",
                std::str::from_utf8(&buffer).unwrap()
            );
        }
    }
    shader
}
pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);

        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "Error al enlazar el programa: {}",
                std::str::from_utf8(&buffer).unwrap()
            );
        }

        // Los shaders pueden ser eliminados después del enlace
        DeleteShader(vertex_shader);
        DeleteShader(fragment_shader);
    }
    program
}
