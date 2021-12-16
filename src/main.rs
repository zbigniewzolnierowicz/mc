use glfw::{Context, WindowEvent};

extern crate gl;
extern crate glfw;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const TICKER_SPEED: f32 = 2.0;

fn main() {
    let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
        Ok(glfw) => glfw,
        Err(_) => {
            panic!("Could not load GLFW, for some reason.");
        },
    };

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.with_primary_monitor(
            |glfw, _| {
                glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "This is a testing window!", glfw::WindowMode::Windowed)
            }
        )
            .expect("Window couldn't be created!");

    window.set_key_polling(true);
    window.make_current();

    glfw.make_context_current(Some(&window));
    gl::load_with(|s| glfw.get_proc_address_raw(s));
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH.try_into().unwrap(), WINDOW_HEIGHT.try_into().unwrap());
    }

    let mut ticker = 0.0;
    let mut flipflopper = false;

    while !window.should_close() {

        if ticker >= 255.0 {
            flipflopper = true;
        } else if ticker <= 0.0 {
            flipflopper = false;
        }

        ticker = if flipflopper {
            ticker - TICKER_SPEED
        } else {
            ticker + TICKER_SPEED
        };

        unsafe {
            gl::ClearColor(ticker / 255.0, ticker / 255.0, 0.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(&mut window, event);
        }
    }
}

fn handle_event(window: &mut glfw::Window, event: WindowEvent) {
    match event {
        WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press, glfw::Modifiers::Control) => {
            window.set_should_close(true);
        },
        WindowEvent::Key(key, _, glfw::Action::Press, modifiers) => {
            println!("Pressed key: {:?}. Modifiers: {:?}.", key, modifiers);
        },
        _ => ()
    }
}
