
pub mod glfw;

pub trait Backend
{
    fn run(&mut self);
    fn end(&mut self);
    fn is_open(&self) -> bool;
}
