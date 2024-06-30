extern crate glium;
extern crate winit;
use glium::Surface;
use winit::event_loop;

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

pub struct Opengldata {
	pub event_loop: event_loop::EventLoop<()>,
 }

const XSIZE: f64 = 800.0;
const YSIZE: f64 = 800.0;


pub fn setup_window() -> Result<Opengldata, std::io::Error>
{
	let event_loop = event_loop::EventLoopBuilder::new()
		.build()
		.expect("event loop building");

	let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
		.with_title("OpenGL window")
		.build(&event_loop);

	let mut frame = display.draw();
	frame.clear_color(0.0, 0.0, 1.0, 1.0);
	frame.finish().unwrap();

	let data: Opengldata = Opengldata {
		event_loop: event_loop,
	};
	return Ok(data);
}

pub fn run_window(event_loop: winit::event_loop::EventLoop<()>)
{
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
