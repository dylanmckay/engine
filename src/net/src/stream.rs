
use std;
use std::io;
use Streamable;

/// A stream can have a single type read from or written to.
/// 
/// It is a direct connection between two nodes.
pub struct Stream<T>
{
    read: Box<io::Read>,
    write: Box<io::Write>,

    phantom: std::marker::PhantomData<T>,
}

impl<T: Streamable> Stream<T>
{
    /// Creates a new stream.
    pub fn new(read: Box<io::Read>,
               write: Box<io::Write>) -> Self {
        Stream {
            read: read,
            write: write,
            phantom: std::marker::PhantomData,
        }
    }

    /// Receives data.
    pub fn receive(&mut self) -> io::Result<T> {
        T::read(&mut self.read)
    }

    /// Sends data.
    pub fn send(&mut self, data: &T) -> io::Result<()> {
        data.write(&mut self.write)
    }

    /// Turns the stream into a stream which uses a different kind of object.
    pub fn morph<V>(self) -> Stream<V>
        where V: Streamable {
        Stream::new(self.read, self.write)
    }
}
