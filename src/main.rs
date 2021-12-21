extern crate glutin as glut;
extern crate gl;
extern crate nalgebra as na;

mod shader;

use std::ffi::{CStr, CString};

use glut::{event::{Event, WindowEvent}, event_loop::ControlFlow, dpi::PhysicalSize};
use shader::{Shader, Program};

const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 720;

fn main() {
    let el = glut::event_loop::EventLoop::new();
    let wb = glut::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glut::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let windowed_context = glut::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(8)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glut::GlRequest::Specific(glut::Api::OpenGl, (4, 6)))
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|ptr| windowed_context.get_proc_address(ptr));

    let vertex_shader = Shader::vertex_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let fragment_shader = Shader::fragment_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    shader_program.set_used();

    let triangle_vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0,   1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,   0.0, 0.0, 1.0,   // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo); // Instantiate one buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Bind the buffer to be an array buffer to store the vertices
        gl::BufferData( // Load the data into the buffer
            gl::ARRAY_BUFFER, // target
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid, // Pointer to data
            gl::STATIC_DRAW // Data is used only once
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // Unbind buffer, so we no longer have access to it
    }

    let mut vao: gl::types::GLuint = 0; // Creating a vertex array object
    unsafe {
        // LOCATION

        gl::GenVertexArrays(1, &mut vao); // Generate vertex array object
        gl::BindVertexArray(vao); // Bind the vertex array object so we can work on it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Bind the vertex buffer object, because we need the data from it

        gl::EnableVertexAttribArray(0); // Enable the Position parameter in triangle.vert
        gl::VertexAttribPointer(
            0, // Index of the parameter [(location = ?) in *.vert file]
            3, // Number of components of the parameter (vec3 has 3 components, etc.)
            gl::FLOAT, // data type
            gl::FALSE, // Should this parameter be normalized?
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // The next element is 3 elements away from the starting point
            std::ptr::null() as *const gl::types::GLvoid // Starting point is at 0
        );
        
        // END LOCATION

        // COLOR

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        // END COLOR

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // Unbind VBO
        gl::BindVertexArray(0); // Unbind VAO
    }

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    let PhysicalSize { width, height } = windowed_context.window().inner_size();
                    unsafe {
                        gl::Viewport(0, 0, width.try_into().unwrap(), height.try_into().unwrap());
                    }
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                _ => ()
            },
            Event::MainEventsCleared => {
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    shader_program.set_used();
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(
                        gl::TRIANGLES, // mode
                        0, // starting index in the enabled arrays
                        3 // number of indices to be rendered
                    );
                }
                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
