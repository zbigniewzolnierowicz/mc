extern crate glutin;
extern crate gl;

use glutin::{event::{Event, WindowEvent, KeyboardInput, ElementState::Pressed}, event_loop::ControlFlow};
use rand::Rng;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const TICKER_SPEED: f32 = 2.0;

struct Color(f32, f32, f32, f32);

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(8)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|ptr| windowed_context.get_proc_address(ptr));
    let mut rng = rand::thread_rng();
    
    let mut current_color: Color = Color(0.3, 0.3, 0.3, 1.0);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::Button { state: Pressed, .. } => {
                    current_color = Color(rng.gen(), rng.gen(), rng.gen(), 1.0);
                    windowed_context.window().request_redraw();
                },
                glutin::event::DeviceEvent::Key(KeyboardInput { state: Pressed, virtual_keycode: key, .. }) => println!("Key pressed: {:?}", key),
                _ => ()
            },
            Event::MainEventsCleared => {
                current_color = Color(rng.gen(), rng.gen(), rng.gen(), 1.0);
                windowed_context.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(current_color.0, current_color.1, current_color.2, current_color.3);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}