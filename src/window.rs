extern crate winit;
use glium::{vertex, Surface};

use crate::parsing::Objdata;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
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

fn convert_to_vertexstruct(vertex_data: Vec<(f64, f64, f64)>) -> Vec<Vertex>
{
	vertex_data.into_iter().map(|(x, y, z)| {
		Vertex {
			position: [x as f32, y as f32, z as f32],
			color: [x as f32, y as f32, z as f32],
		}
	}).collect()
}

pub fn run_window(data: Objdata)
{
	let event_loop = winit::event_loop::EventLoopBuilder::new()
		.build()
		.expect("event loop building");

	let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
		.with_title("OpenGL window")
		.build(&event_loop);

	implement_vertex!(Vertex, position, color);

	let shape :Vec<Vertex> = convert_to_vertexstruct(data.vertex);

	/*for i in 0..shape.len()
	{
		println!("vertex: x:{:?}, y:{:?}, z:{:?}", shape[i].position[0], shape[i].position[1], shape[i].position[2]);
	}*/

	let pos = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();

	let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,&convert_to_face(data.face)).unwrap();
	let vertex_shader_src = r#"
	#version 330

    in vec3 position;
	in vec3 color;      // our new attribute
	out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {
		vertex_color = color;
        gl_Position = matrix * vec4(position, 1.0);
    }
	"#;

	let fragment_shader_src = r#"
	#version 330

    in vec3 vertex_color;
	out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);
    }
	"#;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	let mut t: f32 = 0.0;
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

					let uniforms = uniform! {
						matrix: [
							[t.cos() * 0.1, t.sin() * 0.1 , 0.0, 0.0],
							[t.sin() * 0.1, t.cos() * 0.1, 0.0, 0.0],
							[0.0, 0.0, 0.1, 0.0],
							[0.0, 0.0, 0.0, 1.0f32]
						]
					};

					let mut frame = display.draw();
					frame.clear_color(0.0, 0.0, 1.0, 1.0);
					frame.draw(&pos, &indices, &program, &uniforms,
						&Default::default()).unwrap();
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
