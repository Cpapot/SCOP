extern crate winit;
use glium::Surface;

use crate::parsing::Objdata;

pub fn run_window(data: Objdata)
{
	let event_loop = winit::event_loop::EventLoopBuilder::new()
		.build()
		.expect("event loop building");

	let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
		.with_title("OpenGL window")
		.build(&event_loop);

	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 2],
		color: [f32; 3],
	}
	implement_vertex!(Vertex, position, color);

	let vertex1 = Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0]};
	let vertex2 = Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0]};
	let vertex3 = Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] };
	let shape = vec![vertex1, vertex2, vertex3];


	let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
	#version 330

    in vec3 position;
	in vec3 color;      // our new attribute
	out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {
		vertex_color = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
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
							[ t.cos(), t.sin(), 0.0, 0.0],
							[-t.sin(), t.cos(), 0.0, 0.0],
							[0.0, 0.0, 1.0, 0.0],
							[0.0, 0.0, 0.0, 1.0f32],
						]
					};

					let mut frame = display.draw();
					frame.clear_color(0.0, 0.0, 1.0, 1.0);
					frame.draw(&vertex_buffer, &indices, &program, &uniforms,
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
