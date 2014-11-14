extern crate gl;
extern crate glfw;
extern crate native;

use glfw::Context;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile
    glfw.window_hint(glfw::ContextVersion(4, 1));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw.create_window(1024, 768, "Stainless", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    // Window configuration
    window.set_key_polling(true);
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    // Loop until the user closes the window
    while !window.should_close() {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

        // Draw
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    println!("{}", event);
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
