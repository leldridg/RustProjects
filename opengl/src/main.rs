use std::{f32::consts::PI, time::Instant};
use sdl2::event::{Event, WindowEvent};

mod winsdl;
use winsdl::Winsdl;

mod objects;
use objects::*;

mod transform;
use transform::*;

fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();
    unsafe { gl::Viewport(0, 0, 800, 600); }
    
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

    let mut model_matrix = Mat3::new();

    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();

    unsafe { 
        gl::Uniform2f(u_resolution.id, 800., 600.);

        gl::UniformMatrix3fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
    }

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    // my code
    let speed = 0.0001;
    let mut position_x = 0.0;
    let direction = -1.0;

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                            gl::Uniform2f(u_resolution.id, width as f32, height as f32);
                        }
                    }
                }
                _ => { },
            }
        }
        
        position_x += speed * direction;

        //println!("movement: {}", speed * seconds_elapsed as f32);
        model_matrix.translate(speed as f32, 0.0); 

        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = trianglefan(seconds_elapsed + 3);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

            gl::UniformMatrix3fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());

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