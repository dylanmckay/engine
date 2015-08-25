

pub mod gl;

use num::Num;

/// A rectangular region of the screen.
pub trait Viewport<T: Num>
{
    type Canvas;

    /// Gets the center.
    fn center(&self) -> (T,T);
    /// Gets the half extents.
    fn half_extents(&self) -> (T,T);
    /// Begin rendering into the viewport.
    fn begin(&self) -> Self::Canvas;

    /// Gets the top-left point.
    fn top_left(&self) -> (T,T) {
        let (cx,cy) = self.center();
        let (hx,hy) = self.half_extents();
        (cx-hx, cy-hy)
    }

    /// Gets the width and height.
    fn dimensions(&self) -> (T,T) {
        let two = T::one()+T::one();
        let (hx,hy) = self.half_extents();
        (hx*two, hy*two)
    }
}

/// Specifies which faces should be culled.
pub enum CullingMode
{
    /// Cull front-facing faces.
    Front,
    /// Cull back-facing faces.
    Back,
}
