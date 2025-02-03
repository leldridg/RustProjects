mod winsdl;
use winsdl::Winsdl;
mod objects;
use objects::*;


use sdl2::event::Event;

fn main() {
    let mut winsdl = Winsdl::new(1000, 1000).unwrap();
    unsafe { gl::Viewport(0, 0, 1000, 1000); }
    
    let program = create_program().unwrap();
    program.set();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5,
        0.0, -0.5,
        0.5, 0.5
    ];

    let indices: Vec<u32> = vec! [ 0, 1, 2 ];

    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => { },
            }
        }

        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _
            );
        }

        winsdl.window.gl_swap_window(); // update display
    }
}
