
pub mod glfw;

/// The default window title.
const DEFAULT_TITLE: &'static str = "Engine";

pub trait Backend
{
    fn run(&mut self);
    fn end(&mut self);
    fn is_open(&self) -> bool;
    /// Gets the width and height in pixels.
    fn dimensions(&self) -> (u32,u32);
    /// Sets the title of the window (if possible).
    fn set_title(&mut self, title: &str);
}
