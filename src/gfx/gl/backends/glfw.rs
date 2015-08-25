
extern crate glfw;

use gfx;
use gfx::gl;

use std::sync::mpsc::Receiver;

pub struct Backend
{
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64,glfw::WindowEvent)>,
}

impl Backend
{
    pub fn new() -> Backend {
        use self::glfw::Context;


        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(500,500,
                                                      gl::backends::DEFAULT_TITLE,
                                                      glfw::WindowMode::Windowed)
                                       .expect("Failed to create GLFW window");

        gfx::gl::gl::load_with(|s| window.get_proc_address(s));

        window.set_key_polling(true);
        window.make_current();

        Backend {
            glfw: glfw,
            window: window,
            events: events,
        }
    }
}

impl gfx::gl::Backend for Backend
{
    fn run(&mut self) {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events);
    }

    fn end(&mut self) {
        use self::glfw::Context;
        self.window.swap_buffers()
    }

    fn is_open(&self) -> bool {
        !self.window.should_close()
    }

    fn dimensions(&self) -> (u32,u32) {
        let (width,height) = self.window.get_size();
        (width as u32, height as u32)
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }
}
