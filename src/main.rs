use std::error::Error;
use std::ffi::CString;
use std::iter::repeat;
use std::mem::size_of;
use std::ptr::{null, null_mut};
use std::str::from_utf8;

use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use glfw::Context;

type MyResult<T> = Result<T, Box<dyn Error>>;

//
// A triangle
//
static VERTEX_DATA: [GLfloat; 15] = [
    // X   Y    R    G    B
    0.0, 0.5, 1.0, 0.0, 0.0, // p1
    0.5, -0.5, 0.0, 1.0, 0.0, // p2
    -0.5, -0.5, 0.0, 0.0, 1.0, // p3
];

//
// Vertex Shader
//
static VS_SRC: &str = r#"
    #version 330

    in vec2 position;
    in vec3 color;

    out Data {
        vec3 color;
    } data;

    void main() {
        data.color = color;
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

//
// Fragment Shader
//
static FS_SRC: &str = r#"
    #version 330

    in Data {
        vec3 color;
    } data;

    out vec4 colorOut;

    void main() {
        colorOut = vec4(data.color, 0.0);
    }
"#;

fn main() -> MyResult<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    // Choose a GL profile
    use glfw::WindowHint;
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create Window
    let (mut window, events) = glfw
        .create_window(1024, 768, "Stainless", glfw::WindowMode::Windowed)
        .ok_or("Failed to create GLFW window.")?;

    // Window configuration
    window.set_key_polling(true);
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    let (vs, fs, program, mut vao, mut vbo);

    // OpenGL configuration
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::ClearDepth(1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);

        // Create GLSL shaders
        vs = compile_shader(VS_SRC, gl::VERTEX_SHADER)?;
        fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER)?;
        program = link_program(vs, fs)?;

        vao = 0;
        vbo = 0;

        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        let color_out = CString::new("colorOut")?;
        gl::BindFragDataLocation(program, 0, color_out.as_ptr());

        // Specify the layout of the vertex data
        let position = CString::new("position")?;
        let pos_attr = gl::GetAttribLocation(program, position.as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            5 * size_of::<GLfloat>() as GLsizei,
            null(),
        );

        let color = CString::new("color")?;
        let color_attr = gl::GetAttribLocation(program, color.as_ptr());
        gl::EnableVertexAttribArray(color_attr as GLuint);
        gl::VertexAttribPointer(
            color_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            5 * size_of::<GLfloat>() as GLsizei,
            null::<std::ffi::c_void>().offset(2 * size_of::<GLfloat>() as isize),
        );
    }

    // Loop until the user closes the window
    while !window.should_close() {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Draw
        unsafe {
            // Clear the screen to black
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }

    Ok(())
}

unsafe fn compile_shader(src: &str, shader_type: GLenum) -> MyResult<GLuint> {
    let shader = gl::CreateShader(shader_type);

    // Attempt to compile the shader
    let src = CString::new(src)?;
    gl::ShaderSource(shader, 1, &src.as_ptr(), null());
    gl::CompileShader(shader);

    // Get the compile status
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf: Vec<u8> = repeat(0).take(len as usize - 1).collect(); // subtract 1 to skip the trailing null character
        gl::GetShaderInfoLog(shader, len, null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{:?}", from_utf8(&buf)?);
    }

    Ok(shader)
}

unsafe fn link_program(vs: GLuint, fs: GLuint) -> MyResult<GLuint> {
    let program = gl::CreateProgram();

    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);

    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf: Vec<u8> = repeat(0).take(len as usize - 1).collect(); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{:?}", from_utf8(&buf)?);
    }

    Ok(program)
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    println!("{:?}", event);
    if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
        window.set_should_close(true)
    }
}
