extern crate glutin;
extern crate gl;

use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub struct Opengldata {
	pub event_loop: glutin::event_loop::EventLoop<()>,
	pub windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
 }


const XSIZE: f64 = 800.0;
const YSIZE: f64 = 600.0;


pub fn setup_window() -> Result<Opengldata, std::io::Error>
{
	let wb = WindowBuilder::new()
		.with_title("Fenêtre OpenGL")
		.with_inner_size(LogicalSize::new(XSIZE, YSIZE));

	let event_loop = glutin::event_loop::EventLoop::new();

	let windowed_context = ContextBuilder::new()
		.with_vsync(true)
		.build_windowed(wb, &event_loop)
		.unwrap();

	let data: Opengldata = Opengldata {
		event_loop: event_loop,
		windowed_context: unsafe {windowed_context.make_current().unwrap()},
	};

	gl::load_with(|symbol| data.windowed_context.get_proc_address(symbol) as *const _);

	unsafe {gl::ClearColor(0.1, 0.2, 0.3, 1.0);}
	return Ok(data);
}

pub fn run_window(windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>, event_loop: glutin::event_loop::EventLoop<()>)
{
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event
		{
			Event::WindowEvent { event, .. } => match event
			{WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,_ => (),},
			Event::RedrawRequested(_) =>
			{
				unsafe
				{
					gl::Clear(gl::COLOR_BUFFER_BIT);
					//shader
				}
				windowed_context.swap_buffers().unwrap();
			},_ => (),
		}
	});
}
