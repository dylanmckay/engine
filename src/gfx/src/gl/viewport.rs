
use gl;

/// A rectangular portion of the screen.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Viewport
{
    center: (u32,u32),
    half_extents: (u32,u32),
}

impl ::Viewport<u32> for Viewport
{
    type Canvas = gl::Canvas;

    /// Creates a new viewport.
    fn new(center: (u32,u32), half_extents: (u32,u32)) -> Self {
        Viewport {
            center: center,
            half_extents: half_extents,
        }
    }


    fn center(&self) -> (u32,u32) { self.center }
    fn half_extents(&self) -> (u32,u32) { self.half_extents }

    fn begin(self) -> gl::Canvas {
        gl::Canvas::new(self)
    }
}
