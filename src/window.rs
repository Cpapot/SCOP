extern crate winit;
use glium::{vertex, Surface};

use crate::parsing::Objdata;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	normal: [f32; 3],
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn get_perspective(width: u32, height: u32) -> [[f32; 4]; 4]
{
	let perspective = {
		let aspect_ratio = height as f32 / width as f32;

		let fov: f32 = 3.141592 / 3.0;
		let zfar = 1024.0;
		let znear = 0.1;

		let f = 1.0 / (fov / 2.0).tan();

		[
			[f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
			[         0.0         ,     f ,              0.0              ,   0.0],
			[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
			[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
		]
	};
	return perspective;
}

fn convert_to_face(face: Vec<(f64, f64, f64)>) -> Vec<u16>
{
	let mut vec :Vec<u16> = vec![];
	for i in 0..face.len()
	{
		vec.push(face[i].0 as u16);
		vec.push(face[i].1 as u16);
		vec.push(face[i].2 as u16);
	}
	return vec;
}

fn convert_to_vertexstruct(data: Vec<(f64, f64, f64)>) -> Vec<Vertex> {
    data.into_iter().map(|(x, y, z)| Vertex { position: [x as f32, y as f32, z as f32], normal: [0.0, 0.0, 0.0] }).collect()
}

pub fn run_window(data: Objdata)
{
	let event_loop = winit::event_loop::EventLoopBuilder::new()
		.build()
		.expect("event loop building");

	let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
		.with_title("OpenGL window")
		.build(&event_loop);

	implement_vertex!(Vertex, position, normal);

    let mut shape: Vec<Vertex> = convert_to_vertexstruct(data.vertex);
    let mut norm: Vec<Vertex> = convert_to_vertexstruct(data.normal);

    // Assurez-vous que shape et norm ont la même longueur
    let min_len = shape.len().min(norm.len());
    shape.truncate(min_len);
    norm.truncate(min_len);

    // Assigning normals to vertices
    for i in 0..min_len {
        shape[i].normal = norm[i].position;
    }

	/*for i in 0..norm.len()
	{
		println!("vertex: x:{:?}, y:{:?}, z:{:?}", norm[i].position[0], norm[i].position[1], norm[i].position[2]);
	}*/

	let pos = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,&data.indexs).unwrap();
	let vertex_shader_src = r#"
	#version 330

    in vec3 position;
	in vec3 normal;

	out vec3 v_normal;

	uniform mat4 perspective;
	uniform mat4 view;
	uniform mat4 model;

	void main() {
		mat4 modelview = view * model;
		v_normal = transpose(inverse(mat3(modelview))) * normal;
		gl_Position = perspective * modelview * vec4(position, 1.0);
	}
	"#;

	let fragment_shader_src = r#"
	#version 330

	in vec3 v_normal;

	out vec4 color;

	uniform vec3 u_light;

    void main() {
		float brightness = dot(normalize(v_normal), normalize(u_light));
		vec3 dark_color = vec3(0.6, 0.0, 0.0);
		vec3 regular_color = vec3(1.0, 0.0, 0.0);
		color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
	"#;

	let mut t: f32 = 0.1;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	event_loop.run(move |event, window_target|
	{
		match event
		{
			winit::event::Event::WindowEvent { event, .. } =>
			match event
			{
				winit::event::WindowEvent::CloseRequested => window_target.exit(),
				winit::event::WindowEvent::Resized(window_size) => {display.resize(window_size.into());},
				winit::event::WindowEvent::RedrawRequested => {
					t += 0.02;

					let matrix = [
						[0.01 , 0.0, 0.0, 0.0],
						[0.0, 0.01, 0.0, 0.0],
						[0.0, 0.0, 0.01, 0.0],
						[0.0, 0.0, 2.0, 1.0f32]
					];

					let light = [-1.0, 0.4, 0.9f32];

					let params = glium::DrawParameters {
						depth: glium::Depth {
							test: glium::draw_parameters::DepthTest::IfLess,
							write: true,
							.. Default::default()
						},
						.. Default::default()
					};
					let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

					let mut frame = display.draw();
					frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
					frame.draw(&pos, &indices, &program, &uniform! { model: matrix, view: view, perspective: get_perspective(frame.get_dimensions().0, frame.get_dimensions().1) ,u_light: light }, &params).unwrap();
					frame.finish().unwrap();
				},
				_ => (),
			},
			winit::event::Event::AboutToWait => {
				_window.request_redraw();
			},
			_ => (),
		};
	}).unwrap();
}
