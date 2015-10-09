
use super::{Buffer,DEFAULT_BUFFER_SIZE};
use std;

/// A set of mesh buffers.
pub struct StaticData<I,V>
{
    pub data: Vec<Buffer<I,V>>,
}

impl<I,V> StaticData<I,V>
{
    pub fn new(data: Vec<Buffer<I,V>>) -> Self {
        StaticData {
            data: data,
        }
    }

    pub fn empty() -> Self {
        StaticData::new(Vec::new())
    }

    pub fn buffers<'a>(&'a self) -> std::slice::Iter<'a, Buffer<I,V>> {
        self.data.iter()
    }
}

impl<I,V> Default for StaticData<I,V>
{
    fn default() -> Self {
        Self::empty()
    }
}

/// A mesh builder.
/// Note that indices are stored in the builder
/// exactly as they are given. They are only adjusted
/// for the split it buffers when a mesh is built.
///
/// TODO: split meshes up into multiple buffers.
pub struct StaticBuilder<I, V>
{
    #[allow(dead_code)]
    buffer_size: usize,
    indices: Vec<I>,
    vertices: Vec<V>,
}

impl<I,V> StaticBuilder<I,V>
{
    pub fn new() -> Self {

        StaticBuilder {
            buffer_size: DEFAULT_BUFFER_SIZE,
            indices: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn feed_indices<T>(&mut self, it: T)
        where T: Iterator<Item=I> {

        self.indices.extend(it)
    }

    pub fn feed_vertices<T>(&mut self, it: T)
        where T: Iterator<Item=V> {

        self.vertices.extend(it)
    }
}

impl<I,V> Into<StaticData<I,V>> for StaticBuilder<I,V>
{
    fn into(self) -> StaticData<I,V> {
        let buffer = Buffer::new(self.indices, self.vertices);
        let data = StaticData::new(vec![buffer]);

        data
        
    }
}
