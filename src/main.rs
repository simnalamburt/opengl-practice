extern crate native;
extern crate glfw;

use glfw::Context;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    // Listen to the user's keyboard input
    window.set_key_polling(true);
    // Make the window's context current
    window.make_current();

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
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
