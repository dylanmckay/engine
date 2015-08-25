
use gfx;
use gfx::gl;
use num::Num;

/// A rectangular portion of the screen.
pub struct Viewport<T>
{
    center: (T,T),
    half_extents: (T,T),
}

impl<T: Num> Viewport<T>
{
    /// Gets a viewport filling the entier area.
    pub fn entire_area() -> Self {
        Viewport {
            center: (T::zero(),T::zero()),
            half_extents: (T::one(),T::one()),
        }
    }

    /// Creates a new viewport.
    pub fn new(center: (T,T), half_extents: (T,T)) -> Self {
        Viewport {
            center: center,
            half_extents: half_extents,
        }
    }
}

impl<T: Num> gfx::Viewport<T> for Viewport<T>
{
    type Canvas = gl::Canvas;

    fn center(&self) -> (T,T) { self.center }
    fn half_extents(&self) -> (T,T) { self.half_extents }

    fn begin(&self) -> gl::Canvas {
        unimplemented!();
    }
}
