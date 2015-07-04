
const DEFAULT_BUFFER_SIZE: usize = 4096;

/// A hardware buffer.
pub struct Buffer<T>
{
    pub buffer: Vec<T>,
}

impl<T> Buffer<T>
{
    pub fn new(buffer: Vec<T>) -> Self {
        Buffer {
            buffer: buffer,
        }
    }

    pub fn empty() -> Self {
        Buffer::new(Vec::new())
    }
}

impl<T> From<Vec<T>> for Buffer<T>
{
    fn from(buffer: Vec<T>) -> Self {
        Buffer {
            buffer: buffer,
        }
    }
}

/// A set of hardware buffers.
pub struct Data<T>
{
    pub buffers: Vec<Buffer<T>>,
}

impl<T> Data<T>
{
    pub fn new(buffers: Vec<Buffer<T>>) -> Self {
        Data {
            buffers: buffers,
        }
    }

    pub fn empty() -> Self {
        Data::new(Vec::new())
    }
}

impl<T> From<Vec<T>> for Data<T>
{
    fn from(data: Vec<T>) -> Self {

        let buffer: Buffer<T> = data.into();
        
        let mut buffers = Vec::new();
        buffers.push(buffer);

        Data {
            buffers: buffers,
        }
    }
}

/// A mesh.
pub struct Mesh<I, V>
{
    pub indices: Data<I>,
    pub vertices: Data<V>,
}

impl<I,V> Mesh<I,V>
{
    pub fn new(indices: Data<I>, vertices: Data<V>) -> Self {
        Mesh {
            indices: indices,
            vertices: vertices,
        }
    }

    pub fn empty() -> Self {
        Mesh::new(Data::empty(), Data::empty())
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

impl<I, V> Into<Mesh<I,V>> for Builder<I,V>
{
    fn into(self) -> Mesh<I,V> {
        let indices = self.indices.into();
        let vertices = self.vertices.into();

        Mesh::new(indices, vertices)
    }
}
