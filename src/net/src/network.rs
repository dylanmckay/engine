
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Id(u32);

/// A node in a network.
pub trait Node
{
    /// Gets the ID of the node.
    fn id(&self) -> Id;
}

/// A network.
pub trait Network
{
    type Node: Node;
}
