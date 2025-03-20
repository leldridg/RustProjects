use glam::{Mat4, Vec3};

use super::graphics::*;

pub struct Object {
    vbo: Vbo,
    vao: Vao,
    ibo: Ibo,
    model_matrix: Mat4,
    center: Vec3,
    vertex_count: i32
}

impl Object {
    pub fn new(vertices: &Vec<Vec3>, indices: &Vec<u32>, center: Vec3) -> Self {
        let vbo = Vbo::gen();
        vbo.set(vertices);

        let vao = Vao::gen();
        vao.set();

        let ibo = Ibo::gen();
        ibo.set(indices);

        let model_matrix = Mat4::from_translation(center);

        Object {
            vbo,
            vao,
            ibo,
            model_matrix,
            center,
            vertex_count: indices.len() as i32,
        }
    }

    pub fn set_model_matrix(&mut self, matrix: Mat4) {
        self.model_matrix = matrix;
    }

    pub fn render(&self, u_model_matrix: &Uniform) {
        unsafe {
            gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, self.model_matrix.to_cols_array().as_ptr());
            gl::DrawElements(gl::TRIANGLES, self.vertex_count, gl::UNSIGNED_INT, 0 as *const _,);
        }
    }
}