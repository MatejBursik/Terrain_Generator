use gl::types::*;

// Vertex Array Object
/* Example:
let vao = ArrayObject::new();
vao.bind();
*/

pub struct ArrayObject {
    id: GLuint
}

impl ArrayObject {
    pub fn new() -> ArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        ArrayObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}