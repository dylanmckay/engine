
use std;

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

/// A set of mesh buffers.
pub struct Data<I,V>
{
    pub data: Vec<Buffer<I,V>>,
}

impl<I,V> Data<I,V>
{
    pub fn new(data: Vec<Buffer<I,V>>) -> Self {
        Data {
            data: data,
        }
    }

    pub fn empty() -> Self {
        Data::new(Vec::new())
    }

    pub fn buffers<'a>(&'a self) -> std::slice::Iter<'a, Buffer<I,V>> {
        self.data.iter()
    }
}

impl<I,V> Default for Data<I,V>
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
pub struct Builder<I, V>
{
    #[allow(dead_code)]
    buffer_size: usize,
    indices: Vec<I>,
    vertices: Vec<V>,
}

impl<I,V> Builder<I,V>
{
    pub fn new() -> Self {

        Builder {
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

impl<I,V> Into<Data<I,V>> for Builder<I,V>
{
    fn into(self) -> Data<I,V> {
        let buffer = Buffer::new(self.indices, self.vertices);
        let data = Data::new(vec![buffer]);

        data
        
    }
}
