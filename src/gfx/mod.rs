

pub mod gl;

/// A rectangular region of the screen.
pub trait Viewport
{
    type Canvas;

    /// Gets the center of the viewport.
    fn center(&self) -> (f32,f32);
    /// Gets the width and height of the viewport.
    fn dimensions(&self) -> (f32,f32);
    /// Begin rendering into the viewport.
    fn begin(&self) -> Self::Canvas;
}

/// Specifies which faces should be culled.
pub enum CullingMode
{
    /// Cull front-facing faces.
    Front,
    /// Cull back-facing faces.
    Back,
}
