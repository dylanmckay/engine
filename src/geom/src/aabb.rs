
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
        where I: Iterator<Item=Vector3<T>> {

        // FIXME: this should not be necessary
        let point_buf: Vec<_> = points.collect();

        let center = num::average(point_buf.iter().cloned());
        let (mut max_x, mut max_y, mut max_z) = (T::zero(),T::zero(),T::zero());

        // find the point furtherest away from the center.
        for Vector3(x,y,z) in point_buf.iter()
                              .map(|&v| (v-center).as_positive()) {
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

    pub fn contains_any<I>(&self, mut points: I) -> bool
        where I: Iterator<Item=Vector3<T>> {
        points.any(|p| self.contains(p))
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

    /// Subdivides the octree up into 8
    /// sub-octrees.
    pub fn subdivide(self) -> Vec<Self> {
        let (cx,cy,cz) = self.center.xyz();
        let qtr_extents = self.quarter_extents();
        let (qx,qy,qz) = qtr_extents.xyz();

        // front boxes
        let fr_tp_lf = Vector3(cx-qx, cy+qy, cz+qz);
        let fr_tp_rt = Vector3(cx+qx, cy+qy, cz+qz);
        let fr_bt_lf = Vector3(cx-qx, cy-qy, cz+qz);
        let fr_bt_rt = Vector3(cx+qx, cy-qy, cz+qz);

        // back boxes
        let bk_tp_lf = Vector3(cx-qx, cy+qy, cz-qz);
        let bk_tp_rt = Vector3(cx+qx, cy+qy, cz-qz);
        let bk_bt_lf = Vector3(cx-qx, cy-qy, cz-qz);
        let bk_bt_rt = Vector3(cx+qx, cy-qy, cz-qz);

        [
            Aabb::new(fr_tp_lf, qtr_extents),
            Aabb::new(fr_tp_rt, qtr_extents),
            Aabb::new(fr_bt_lf, qtr_extents),
            Aabb::new(fr_bt_rt, qtr_extents),

            Aabb::new(bk_tp_lf, qtr_extents),
            Aabb::new(bk_tp_rt, qtr_extents),
            Aabb::new(bk_bt_lf, qtr_extents),
            Aabb::new(bk_bt_rt, qtr_extents),
        ].into_iter().cloned().collect()
    }

    pub fn center(&self) -> Vector3<T> { self.center }
    pub fn half_extents(&self) -> Vector3<T> { self.half_extents }
    pub fn quarter_extents(&self) -> Vector3<T> {
        let two = T::one()+T::one();
        self.half_extents.map(|a| a/two)
    }
}

