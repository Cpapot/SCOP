/*struct Structure {
	field1:i32,
	field2:i32,
	field3:i32
 }

fn main()
{
	let float:i32 = 1.00005 as i32;
	println!("value {}", float);
	let float:i32 = 4.00005 as i32;
	println!("value {}", float);


	let mut test:f32 = 1.00005 as f32;
	println!("value {}", test);
	test = 4.00005 as f32;
	println!("value {}", test);


	let tuple:(f64, f64, f64) = (1.00, 2.0, 3.0);
	println!("value {:?}", tuple);
	println!("value {:?}", tuple.0);

	let arr:[i32; 5] = [1, 2, 3, 4, 5];
	println!("value {:?}", arr);
	println!("value {}", arr.len());
	println!("value {:?}", arr[1]);

	let stru:Structure = Structure{field1:15, field2:25, field3:35};
	println!("value {:?}", stru.field1);

	panic!("This is a panic message");
}*/

/*extern crate glutin;
extern crate gl;

use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    // Création de l'événement et de la fenêtre
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Fenêtre OpenGL")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &event_loop)
        .unwrap();

    // Rendre le contexte OpenGL actuel
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // Charger les fonctions OpenGL
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    // Configuration OpenGL
    unsafe {
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
    }

    // Boucle d'événements
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // Rendu OpenGL
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}*/

extern crate rfd;

fn main() {
    // Ouvrir un explorateur de fichiers pour choisir un fichier
    let file = rfd::FileDialog::new()
        .set_title("Choisir un fichier")
        .pick_file();

    match file {
        Some(path) => println!("Fichier sélectionné : {:?}", path),
        None => println!("Aucun fichier sélectionné"),
    }
}
