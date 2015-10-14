
use Transform3;
use super::Buffer;
use math::Matrix;
use std;

/// Animated mesh data.
pub struct AnimatedData<I,V>
{
    joints: Vec<Joint<I,V>>,
}

impl<I,V> AnimatedData<I,V>
{
    /// Creates a new animated mesh data object.
    pub fn new<A>(joints: A) -> Self
        where A: Iterator<Item=Joint<I,V>> {

        AnimatedData {
            joints: joints.collect(),
        }
    }

    /// Gets the joints that make up the animation.
    pub fn joints<'a>(&'a self) -> std::slice::Iter<'a,Joint<I,V>> {
        self.joints.iter()
    }

    /// Gets the joints (mutably) that make up the animation.
    pub fn joints_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Joint<I,V>> {
        self.joints.iter_mut()
    }
}

/// A bone joint.
pub struct Joint<I,V>
{
    children: Vec<Joint<I,V>>,
    buffer: Vec<Buffer<I,V>>,

    transform: Transform3,
}

impl<I,V> Joint<I,V>
{
    /// Creates an empty joint.
    pub fn empty() -> Self {
        Joint {
            children: Vec::new(),
            buffer: Vec::new(),

            transform: Transform3::identity(),
        }
    }

    /// Gets the child joints.
    pub fn children<'a>(&'a self) -> std::slice::Iter<'a,Self> {
        self.children.iter()
    }

    /// Gets the child joints mutably.
    pub fn children_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Self> {
        self.children.iter_mut()
    }

    /// Gets an iterator to the mesh buffer.
    pub fn buffer<'a>(&'a self) -> std::slice::Iter<'a,Buffer<I,V>> {
        self.buffer.iter()
    }

    /// Gets an iterator to the mesh buffer mutably.
    pub fn buffer_mut<'a>(&'a mut self) -> std::slice::IterMut<'a,Buffer<I,V>> {
        self.buffer.iter_mut()
    }
 
    /// Gets the graphics transformation.
    pub fn transform(&self) -> Transform3 { self.transform }
    /// Gets the graphics transformation mutably.
    pub fn transform_mut(&mut self) -> &mut Transform3 {
        &mut self.transform
    }
}
