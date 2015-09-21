
use num::{self,Num};
use math::Vector3;

/// An axis-aligned bounding box.
#[derive(Copy,Clone)]
pub struct Aabb<T: Num>
{
    center: Vector3<T>,
    half_extents: Vector3<T>,
}

impl<T: Num> Aabb<T>
{
    pub fn new(center: Vector3<T>,
               half_extents: Vector3<T>) -> Self {
        Aabb {
            center: center,
            half_extents: half_extents,
        }
    }

    /// Creates a bounding box containing a set of points.
    pub fn containing<I>(points: I) -> Self
        where I: Iterator<Item=Vector3<T>> + Clone{

        let center = num::average(points.clone());
        let (mut max_x, mut max_y, mut max_z) = (T::zero(),T::zero(),T::zero());

        // find the point furtherest away from the center.
        for Vector3(x,y,z) in points
                              .map(|v| (v-center).as_positive()) {
            if x > max_x { max_x = x }
            if y > max_y { max_y = y }
            if z > max_z { max_z = z }
        }

        let half_extents = Vector3(max_x, max_y, max_z);

        Aabb::new(center, half_extents)
    }

    /// Checks if the box contains a point.
    pub fn contains(&self, point: Vector3<T>) -> bool {
        let (dx,dy,dz) = (point - self.center).as_positive().into();
        let (hx,hy,hz) = self.half_extents.into();

        (dx <= hx) && (dy <= hy) && (dz <= hz)
    }

    /// Grows the bounding box to include a point if necessary.
    pub fn grow(self, point: Vector3<T>) -> Self {
        let (dx,dy,dz) = (point - self.center).as_positive().into();
        let (mut hx, mut hy, mut hz) = self.half_extents.into();

        if dx > hx { hx = dx }
        if dy > hy { hy = dy }
        if dz > hz { hz = dz }

        let half_extents = Vector3(hx, hy, hz);
        Aabb::new(self.center, half_extents)
    }

    pub fn center(&self) -> Vector3<T> { self.center }
    pub fn half_extents(&self) -> Vector3<T> { self.half_extents }
}

