
pub use self::glfw::Backend as Glfw;
pub mod glfw;

use input::Event;

pub fn default() -> Glfw {
    Glfw::new()
}

/// The default window title.
const DEFAULT_TITLE: &'static str = "Engine";

pub trait Backend
{
    fn run(&mut self, events: &mut Vec<Event>);
    fn end(&mut self);
    fn is_open(&self) -> bool;
    /// Gets the width and height in pixels.
    fn dimensions(&self) -> (u32,u32);
    /// Sets the title of the window (if possible).
    fn set_title(&mut self, title: &str);
    
    fn set_mouse_pos(&mut self, pos: (u32,u32));
    fn set_cursor_visible(&mut self, visible: bool);
}
