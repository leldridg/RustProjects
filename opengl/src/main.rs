use std::{f32::consts::PI, time::Instant};
use sdl2::event::{Event, WindowEvent};

mod winsdl;
use winsdl::Winsdl;

mod objects;
use objects::*;

mod transform;
use transform::*;

mod vertex;
use vertex::*;

fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();
    unsafe { gl::Viewport(0, 0, 800, 600); }
    
    let mut max_uniforms: gl::types::GLint = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_UNIFORM_VECTORS, &mut max_uniforms); }
    println!("Max uniforms: {}", max_uniforms);
    println!("Maximum number of uniforms: {}", std::mem::size_of::<Vertex>());

    let program = create_program().unwrap();
    program.set();

    // let vertices: Vec<f32> = vec![
    //     -0.5, -0.5,
    //     0.0, -0.5,
    //     0.5, 0.5
    // ];

    // let indices: Vec<u32> = vec! [ 0, 1, 2 ];

    let (mut vertices, mut indices) = triangle_fan_3D(3, 6);

    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    // let mut model_matrix = Mat4::new();
    // let mut view_matrix = Mat4::new();
    let mut model_matrix: [Mat4 ; 6] = [Mat4::new(); 6];
    let mut view_matrix: Mat4;
    let mut projection_matrix: Mat4 = Mat4::new();
    projection_matrix.project_orthographic(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_projection_matrix = Uniform::new(program.id(), "u_projection_matrix").unwrap();

    unsafe { 
        gl::Uniform2f(u_resolution.id, 800., 600.);

        // mat3
        // gl::UniformMatrix3fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
        // gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
        // gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::TRUE, view_matrix.ptr());
        gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::TRUE, projection_matrix.ptr());

        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::BlendEquation(gl::FUNC_ADD);
    }

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    // my code
    let x_velocity = 0.0001;
    let y_velocity = 0;

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

        // translating with mat3
        // model_matrix.translate(x_velocity as f32, 0.0); 
        

        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = triangle_fan_3D(seconds_elapsed + 3, 6);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

            let time_mod = start.elapsed().as_secs_f32() % 6.0;

            for (i, m) in model_matrix.iter_mut().enumerate() {
                *m = Mat4::new();
                //m.translate(0.001, 0., 0.);
                //m.scale((time_mod + 1.0) / 5.0, (time_mod+1.0) / 5.0, 1.0);
                //m.rotate_z(time_mod.powi(2) / 2.);
                m.rotate_x(PI / 6. * i as f32);
            }

            //model_matrix = Mat4::new();
            view_matrix = Mat4::new();
            view_matrix.lookat((time_mod / 3.0 * PI).sin()*0.5, 0.2, (time_mod / 3.0 * PI).cos()*0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            //view_matrix.lookat(0.0, 0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            //model_matrix.rotate_y(0.01);

            // translating with mat3
            //gl::UniformMatrix3fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
            gl::UniformMatrix4fv(u_model_matrix.id, 6, gl::TRUE, model_matrix[0].ptr());
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::TRUE, view_matrix.ptr());

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

fn triangle_fan(n: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = vec![
        Vertex::from_pos(0.0, 0.0),
        Vertex::from_pos(0.5, 0.0),
    ];

    let mut indices: Vec<u32> = vec! [ ];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(Vertex::from_pos(
            angle.cos() * 0.5,
            angle.sin() * 0.5
        ));

        indices.push(0);
        indices.push(m);
        indices.push(m + 1);
    }

    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}

#[allow(non_snake_case)]
fn triangle_fan_3D(n: u32, entities_number: u32) -> (Vec<Vertex>, Vec<u32>) {
    let (vertices, indices) = triangle_fan(n);
    
    if entities_number < 2 {
        return (vertices, indices);
    }

    let mut final_vertices: Vec<Vertex> = vertices.clone();
    let mut final_indices: Vec<u32> = indices.clone();

    (1..entities_number)
        .into_iter()
        .for_each(|id| {
            final_vertices.extend(vertices
                .clone()
                .into_iter()
                .map(|mut vertex| {
                    vertex.entity_id = id;
                    vertex
                })
                .collect::<Vec<Vertex>>()
            );
            final_indices.extend(indices
                .clone()
                .into_iter()
                .map(|val| {
                    val + id * vertices.len() as u32
                })
                .collect::<Vec<u32>>()
            );
        });
    (final_vertices, final_indices)
}