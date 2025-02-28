use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

use gl::types::*;

use cgmath::*;

// Shader Reader
/* Examples:
let mut program = ShaderReader::new("/path/to/vertex_shader.glsl", "/path/to/fragment_shader.glsl");
program.bind();
program.create_uniform("transform");
// program.set_matrix4fv_uniform("transform", some_matrix);
*/

pub struct ShaderReader {
    program_handle: u32,
    uniform_ids: HashMap<String, GLint>
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderReader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderReader {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut vertex_shader_source = String::new();
        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");

        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));
        let mut fragment_shader_source = String::new();
        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderReader {
                program_handle,
                uniform_ids: HashMap::new()
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handle,
                CString::new(uniform_name).unwrap().as_ptr()
            )
        };
        
        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr()
            )
        }
    }
}