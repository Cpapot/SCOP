extern crate winit;
use glium::Surface;

pub fn run_window()
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
	}
	implement_vertex!(Vertex, position);

	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [ 0.0,  0.5] };
	let vertex3 = Vertex { position: [ 0.5, -0.25] };
	let shape = vec![vertex1, vertex2, vertex3];
	let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
	#version 330

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
	"#;

	let fragment_shader_src = r#"
	#version 330

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
	"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	let mut frame = display.draw();
	frame.clear_color(0.0, 0.0, 1.0, 1.0);
	frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
		&Default::default()).unwrap();
	frame.finish().unwrap();

	event_loop.run(move |event, window_target|
	{
		match event
		{
			winit::event::Event::WindowEvent { event, .. } =>
			match event
			{
				winit::event::WindowEvent::CloseRequested => window_target.exit(),
				_ => (),
			},
			_ => (),
		};
	}).unwrap();
}
