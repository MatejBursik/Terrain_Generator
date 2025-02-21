use gl::types::*;

// Buffer Object
/* Example
let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
vbo.bind();
vbo.store_f32_data(&float32_array);
*/
pub struct BufferObject {
    id: GLuint,
    r_type: GLenum,
    usage: GLenum
}

impl BufferObject {
    pub fn new(r_type: GLenum, usage: GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id, r_type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r_type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r_type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r_type,
                (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const _,
                self.usage
            );
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r_type,
                (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const i32 as *const _,
                self.usage
            );
        }
    }
}