extern crate glutin as glut;
extern crate gl;
extern crate nalgebra as na;

use std::ffi::{CStr, CString};

use glut::{event::{Event, WindowEvent}, event_loop::ControlFlow, dpi::PhysicalSize};

const WINDOW_WIDTH: u32 = 1280;
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
                _ => ()
            },
            Event::MainEventsCleared => {
            },
            Event::RedrawRequested(_) => {
                let PhysicalSize { width, height } = windowed_context.window().inner_size();
                unsafe {
                    gl::Viewport(0, 0, width.try_into().unwrap(), height.try_into().unwrap());
                    gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(source: &CStr, shader_type: gl::types::GLenum) -> Result<Self, String> {

        use gl::types::GLint;
    
        let id = unsafe { gl::CreateShader(shader_type) }; // Create a shader ID
    
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()); // Set the source of the shader to be the string we passed
            gl::CompileShader(id); // Compile the shader
        }
    
        let mut is_compilation_success: GLint = 1; // 1 = good, 0 = bad
        unsafe {
            // Check if compilation succeeded
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut is_compilation_success);
        }
        
        // Compilation did not succeed
        if is_compilation_success == 0 {
            let error_string = unsafe {
                let length_of_error_log: GLint = {
                    // Check the length of the error log and save it to length_of_error_log
                    let mut len: GLint = 0;
                    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); 
                    len
                };
        
                // Create a buffer with enough size to hold the log
                let error = make_buffer_string_of_length(length_of_error_log as usize);
    
                // Write the shader logs to the error variable
                gl::GetShaderInfoLog(
                    id, // ID of the compiled shader
                    length_of_error_log,
                    std::ptr::null_mut(), // We don't need to write down the length of the program info log
                    error.as_ptr() as *mut gl::types::GLchar
                );
    
                error
            };
            return Err(error_string.to_string_lossy().into_owned());
        }
    
        Ok(Self {
            id
        })
    }

    pub fn vertex_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn fragment_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        // Delete the shader after we're done with it, to avoid a memory leak
        unsafe { gl::DeleteShader(self.id) };
    }
}

fn make_buffer_string_of_length(len: usize) -> CString {
    // Make a buffer with enough space
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // Fill it with empty spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // Convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id()); // Load shader code into the program
            }
        }

        unsafe {
            gl::LinkProgram(program_id); // Load the program
        }

        let mut success: gl::types::GLint = 1; // 0 = bad, 1 = good
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); // Check if the program linked up properly
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0; // Initialize the length of the error log
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len); // Load the length of the error log
            }

            let error = make_buffer_string_of_length(len as usize); // Create an empty string to hold the error log

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                ); // Load the error log into the error string
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id()); // Remove the shader code, as it has been already loaded into the program
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}