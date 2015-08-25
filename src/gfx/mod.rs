

pub mod gl;

use num::Num;
use math;

/// A rectangular region of the screen.
pub trait Viewport<T: Num> : Sized + Clone
{
    type Canvas;

    /// Creates a new viewport.
    fn new(center: (T,T), half_extents: (T,T)) -> Self;

    /// Gets the center.
    fn center(&self) -> (T,T);
    /// Gets the half extents.
    fn half_extents(&self) -> (T,T);

    fn quarter_extents(&self) -> (T,T) {
        let (hx,hy) = self.half_extents();
        let two = T::one()+T::one();

        (hx/two, hy/two)
    }
    /// Begin rendering into the viewport.
    fn begin(self) -> Self::Canvas;

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

    fn split_half(&self, axis: math::Axis2) -> (Self,Self) {
        let (cx,cy) = self.center();
        let (qx,qy) = self.quarter_extents();

        match axis {
            math::Axis2::Vertical => {
                let c1x = cx - qx;
                let c2x = cx + qx;

                (Self::new( (c1x,cy), (qx,qy) ),
                 Self::new( (c2x,cy), (qx,qy) ))

            },
            _ => unimplemented!(),
        }
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
