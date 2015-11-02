
use std::io;

/// A type which can be streamed.
pub trait Readable : Sized
{
    fn read(read: &mut io::Read) -> io::Result<Self>;
}

/// A type which can be written.
pub trait Writable : Sized
{
    fn write(&self, write: &mut io::Write) -> io::Result<()>;
}

/// A type which may be streamed.
pub trait Streamable : Readable + Writable
{

}
