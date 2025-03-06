use std::{f32::consts::PI, time::Instant};
use sdl2::event::{Event, WindowEvent};
use glam::{Mat4, Vec3, Vec4};

mod winsdl;
use winsdl::Winsdl;

mod objects;
use objects::*;

mod transform;
use transform::*;

mod vertex;
use vertex::*;

fn main() {
    let width: usize = 800;
    let height: usize = 600;
    let mut winsdl = Winsdl::new(width, height).unwrap();
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    
    let mut max_uniforms: gl::types::GLint = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_UNIFORM_VECTORS, &mut max_uniforms); }
    println!("Max uniforms: {}", max_uniforms);
    println!("Maximum number of uniforms: {}", std::mem::size_of::<Vertex>());

    let mut orthographic: bool = false;
    let mut perspective: bool = false;

    let program = create_program().unwrap();
    program.set();

    // let vertices: Vec<f32> = vec![
    //     -0.5, -0.5
    //     0.0, -0.5,
    //     0.5, 0.5
    // ];

    // let indices: Vec<u32> = vec! [ 0, 1, 2 ];

    //let (mut vertices, mut indices) = triangle_fan_3D(3, 6);

    let mut all_vertices: Vec<Vertex> = Vec::new();
    let mut all_indices: Vec<u32> = Vec::new();

    let mut cube_center0 = Vec3::new(0., 0., 0.);
    let (vertices, indices) = cube(0, cube_center0);
    all_vertices.extend(vertices);
    all_indices.extend(indices);

    
    let cube_center1 = Vec3::new(0., 0., 0.);
    let (vertices, indices) = cube(1, cube_center1);
    all_vertices.extend(vertices);
    all_indices.extend(indices);

    let vbo = Vbo::gen();
    vbo.set(&all_vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&all_indices);

    // let mut model_matrix = Mat4::new();
    // let mut view_matrix = Mat4::new();
    let mut model_matrix: [Mat4 ; 2] = [Mat4::IDENTITY; 2];
    let mut view_matrix: Mat4;
    let mut projection_matrix: Mat4 = Mat4::IDENTITY;
    //projection_matrix.project_perspective(-1., 1., -1., 1., 0.2, 2.);
    //projection_matrix.project_orthographic(-1.0, 1.0, -1.0, 1.0, 0.2, 2.0);
    let l = -1.;
    let r = 1.;
    let b = -1.;
    let t = 1.;
    let n = 0.2;
    let f = 2.;

    let aspect_ratio = (r - l) / (t - b);
    println!("Aspect ratio is {} for r {}, l {}, t {}, and b {}", aspect_ratio, r, l, t, b);

    let fov_y=(2.0 * ((t - b) / (2.0 * n)) as f32).atan();
    println!("Fov_y is {} for t {}, b {}, n {}", fov_y, t, b, n);


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
        gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        //gl::Enable(gl::BLEND);
        //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        //gl::BlendEquation(gl::FUNC_ADD);
    }

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    // my code
    let x_velocity = 0.0001;
    let y_velocity = 0;
    
    cube_center0[0] = 1.5; // is this actually cube center coord?
    //model_matrix[0] = Mat4::from_translation(cube_center0);
    model_matrix[0] = Mat4::from_translation(Vec3::new(0.,0.,0.));
    model_matrix[1] = Mat4::from_translation(Vec3::new(0.,0.,0.));
   

    let mut eye_x = 0.0;
    let mut eye_y = 0.0;
    //let mut eye_z = 5.;
    let mut eye_z = 0.5;
    let target_x = 0.;
    let target_y = 0.;
    let mut target_z = 0.;
    let up_x = 0.;
    let mut up_y = 1.;
    let up_z = 0.;

    view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));

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
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            println!("Screen x: {}, Screen y: {}", x, y);
                            // normalize x and y coordinates
                            let norm = Vec4::new(
                                (2. * x as f32) / width as f32 - 1.,
                                1. - (2. * y as f32) / height as f32,
                                -1.,
                                1.
                            );
                            // inverse projection matrix
                            let inverse_projection = projection_matrix.inverse();
                            // multiply inverse projection matrix and (x_n, y_n, z, w)
                            let mut ray_eye = inverse_projection * norm;
                            ray_eye[2] = -1.;
                            ray_eye[3] = 0.;
                            // inverse view matrix
                            let inverse_view = view_matrix.inverse();
                            // multiply inverse view matrix and ray_eye
                            let ray_world = inverse_view * ray_eye;
                            // normalize ray_world
                            let length = (ray_world[0] * ray_world[0] + ray_world[1] * ray_world[1] + ray_world[2] * ray_world[2]).sqrt();
                            let result: [f32; 3];
                            if length != 0. {
                                result = [
                                    ray_world[0] / length,
                                    ray_world[1] / length,
                                    ray_world[2] / length
                                ];
                            } else {
                                result = [-1., -1., -1.];
                            }
                            println!("World x: {}, World y: {}, World z: {}", result[0], result[1], result[2]);
                            cube_center0 = Vec3::new(result[0], result[1], 0.);
                            let test = Vec3::new(ray_world[0], ray_world[1], 0.);
                            println!("x: {}, y: {}", ray_world[0], ray_world[1]);
                            let (vertices, indices) = cube(0, test);
                            all_vertices[0..vertices.len()].clone_from_slice(&vertices);
                            vbo.set(&all_vertices);
                            // model_matrix[0] = Mat4::from_translation(Vec3::new(result[0], result[1], 0.));
                        },
                        _ => { }
                    }
                },
                Event::KeyDown { keycode, .. } => {
                    let amt = 0.1;
                    match keycode {
                        Some(key) => {
                            match key {
                                // cube movement
                                sdl2::keyboard::Keycode::W => {
                                    //model_matrix[0].translate(0., 0., -0.1); 
                                    cube_center0[2] -= 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0);
                                },
                                sdl2::keyboard::Keycode::S => { 
                                    //model_matrix[0].translate(0., 0., 0.1); 
                                    cube_center0[2] += 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0);

                                },
                                sdl2::keyboard::Keycode::A => {
                                    cube_center0[0] -= 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0); 
                                },
                                sdl2::keyboard::Keycode::D => { 
                                    cube_center0[0] += 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0); 
                                },
                                sdl2::keyboard::Keycode::Equals => {
                                    cube_center0[1] += 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0);
                                },
                                sdl2::keyboard::Keycode::Minus => {
                                    cube_center0[1] -= 0.1;
                                    model_matrix[0] = Mat4::from_translation(cube_center0);
                                },
                                // camera movement
                                sdl2::keyboard::Keycode::Left => { 
                                    eye_x += amt;
                                },
                                sdl2::keyboard::Keycode::Right => {
                                    eye_x -= amt;
                                },
                                sdl2::keyboard::Keycode::Up => {
                                    eye_z -= amt;
                                    //up_y -= amt;
                                },
                                sdl2::keyboard::Keycode::Down => {
                                    eye_z += amt;
                                    //up_y += amt;
                                },
                                sdl2::keyboard::Keycode::Comma => {
                                    eye_y -= amt;
                                },
                                sdl2::keyboard::Keycode::Period => {
                                    eye_y += amt;
                                },
                                sdl2::keyboard::Keycode::LeftBracket => {
                                    target_z -= amt;
                                },
                                sdl2::keyboard::Keycode::RightBracket => {
                                    target_z += amt;
                                },
                                // projection
                                sdl2::keyboard::Keycode::P => {
                                    projection_matrix = Mat4::perspective_infinite_rh(fov_y, aspect_ratio, n);
                                    perspective = true;
                                    orthographic = false;
                                },
                                sdl2::keyboard::Keycode::O => {
                                    projection_matrix = Mat4::orthographic_rh_gl(l, r, b, t, n, f);
                                    orthographic = true;
                                    perspective = false;
                                },
                                _ => { }
                            }
                            unsafe {
                                gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
                            }
                            view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));
                            println!("Key pressed: {:?}", key);
                            println!("eye_x: {}, eye_y: {}, eye_z: {}", eye_x, eye_y, eye_z);
                            println!("up_x: {}, up_y: {}, up_z: {}", up_x, up_y, up_z);
                            println!("target_z: {}", target_z);
                            println!("cube_center: {:?}", cube_center0);
                            
                        },
                        None => { }
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

            let time_mod = start.elapsed().as_secs_f32() % 6.0;

            /* 
            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = triangle_fan_3D(seconds_elapsed + 3, 6);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

            for (i, m) in model_matrix.iter_mut().enumerate() {
                *m = Mat4::new();
                //m.translate(0.001, 0., 0.);
                //m.scale((time_mod + 1.0) / 5.0, (time_mod+1.0) / 5.0, 1.0);
                //m.rotate_z(time_mod.powi(2) / 2.);
                m.rotate_x(PI / 6. * i as f32);
            }
            */

            //model_matrix = Mat4::new();
            //view_matrix = Mat4::new();
            //view_matrix.lookat((time_mod / 3.0 * PI).sin()*0.5, 0.2, (time_mod / 3.0 * PI).cos()*0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            //println!("eye_x: {}, eye_y: {}, eye_z: {}", (time_mod / 3.0 * PI).sin()*0.5, 0.2, (time_mod / 3.0 * PI).cos()*0.5);
            //view_matrix.lookat(0.0, 0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            //model_matrix.rotate_y(0.01);

            // translating with mat3
            //gl::UniformMatrix3fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
            gl::UniformMatrix4fv(u_model_matrix.id, 2, gl::FALSE, model_matrix[0].to_cols_array().as_ptr());
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, view_matrix.to_cols_array().as_ptr());

            gl::DrawElements(
                gl::TRIANGLES,
                all_indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _
            );
        }

        winsdl.window.gl_swap_window(); // update display
    }
}

fn cube(entity_id: u32, cube_center: Vec3) -> (Vec<Vertex>, Vec<u32>) {

    let mut vertices: Vec<Vertex> = vec! [
        // front face
        Vertex::from_pos(-0.5 + cube_center[0], -0.5 + cube_center[1], -0.5),
        Vertex::from_pos(-0.5 + cube_center[0], 0.5 + cube_center[1], -0.5),
        Vertex::from_pos(0.5 + cube_center[0], 0.5 + cube_center[1], -0.5),
        Vertex::from_pos(0.5 + cube_center[0], -0.5 + cube_center[1], -0.5),

        // back face
        Vertex::from_pos(0.5 + cube_center[0], -0.5 + cube_center[1], 0.5),
        Vertex::from_pos(0.5 + cube_center[0], 0.5 + cube_center[1], 0.5),
        Vertex::from_pos(-0.5 + cube_center[0], 0.5 + cube_center[1], 0.5),
        Vertex::from_pos(-0.5 + cube_center[0], -0.5 + cube_center[1], 0.5),

    ];

    for vertex in &mut vertices {
        vertex.entity_id = entity_id;
    }

    let mut indices: Vec<u32> = vec! [
        // front face
        0, 1, 2,
        2, 3, 0,

        // back face
        4, 5, 6,
        6, 7, 4,

        // top face
        1, 6, 5,
        5, 2, 1,

        // bottom face
        7, 0, 3,
        3, 4, 7,

        // right face
        3, 2, 5,
        5, 4, 3,

        // left face
        7, 6, 1,
        1, 0, 7,
    ];

    indices = indices.iter().map(|&i| i + 8 * entity_id).collect::<Vec<u32>>();

    (vertices, indices)
}

fn triangle_fan(n: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = vec![
        Vertex::from_pos(0.0, 0.0, 0.0),
        Vertex::from_pos(0.5, 0.0, 0.0),
    ];

    let mut indices: Vec<u32> = vec! [ ];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(Vertex::from_pos(
            angle.cos() * 0.5,
            angle.sin() * 0.5,
            0.0
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