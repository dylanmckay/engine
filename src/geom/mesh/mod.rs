
pub use self::stat::{StaticData,StaticBuilder};
pub use self::anim::AnimatedData;

pub mod stat;
pub mod anim;


const DEFAULT_BUFFER_SIZE: usize = 4096;

/// A mesh buffer.
pub struct Buffer<I,V>
{
    pub indices: Vec<I>,
    pub vertices: Vec<V>,
}

impl<I,V> Buffer<I,V>
{
    pub fn new(indices: Vec<I>, vertices: Vec<V>) -> Self {
        Buffer {
            indices: indices,
            vertices: vertices,
        }
    }

    pub fn empty() -> Self {
        Buffer::new(Vec::new(), Vec::new())
    }
}

impl<I,V> Default for Buffer<I,V>
{
    fn default() -> Self {
        Self::empty()
    }
}

