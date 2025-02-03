mod winsdl;
use std::{f32::consts::PI, time::Instant};

use winsdl::Winsdl;
mod objects;
use objects::*;


use sdl2::event::Event;

fn main() {
    let mut winsdl = Winsdl::new(1000, 1000).unwrap();
    unsafe { gl::Viewport(0, 0, 1000, 1000); }
    
    let program = create_program().unwrap();
    program.set();

    // let vertices: Vec<f32> = vec![
    //     -0.5, -0.5,
    //     0.0, -0.5,
    //     0.5, 0.5
    // ];

    // let indices: Vec<u32> = vec! [ 0, 1, 2 ];

    let (mut vertices, mut indices) = trianglefan(3);

    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

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

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = trianglefan(seconds_elapsed + 3);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

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

fn trianglefan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = vec![
        0.0, 0.0,
        0.5, 0.0,
    ];

    let mut indices: Vec<u32> = vec! [ ];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);

        indices.push(0);
        indices.push(m);
        indices.push(m + 1);
    }

    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}