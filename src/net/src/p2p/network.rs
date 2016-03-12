
use {Stream,Streamable,Id};

/// A node in a network.
pub struct Node<T: Streamable>
{
    /// The ID of the node.
    id: Id,
    /// The IDs of the adjacent nodes.
    neighbours: Vec<Id>,
    /// An optional direct connection to the node.
    connection: Option<Stream<T>>,
}

impl<T> Node<T>
    where T: Streamable
{
    /// Creates a node with no connections.
    pub fn new(id: Id) -> Self {
        Node {
            id: id,
            neighbours: Vec::new(),
            connection: None,
        }
    }

    // TODO: connect and set_connection is confusing.

    /// Adds an adjacent node.
    pub fn connect(&mut self, id: Id) {
        self.neighbours.push(id);
    }

    /// Sets the connection.
    pub fn set_connection(&mut self, con: Option<Stream<T>>) {
        self.connection = con;
    }

    pub fn id(&self) -> Id { self.id }

    pub fn neighbour_ids(&self) -> ::std::slice::Iter<Id> {
        self.neighbours.iter()
    }

    pub fn is_neighbour_of(&self, id: Id) -> bool {
        self.neighbours.iter().any(|&node| node == id)
    }
}

/// A peer-to-peer network.
pub struct Network<T: Streamable>
{
    /// The ID of the local device.
    local_id: Id,
    /// The nodes inside the network.
    nodes: Vec<Node<T>>,
}

impl<T> Network<T>
    where T: Streamable
{
    /// Creates an empty network.
    pub fn empty(local_id: Id) -> Self {
        Network {
            local_id: local_id,
            nodes: Vec::new(),
        }
    }

    /// Adds a node into the network.
    pub fn add(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    /// Gets the neighbours of a node.
    pub fn neighbours(&self, node: &Node<T>) -> ::std::vec::IntoIter<&Node<T>> {
        // TODO: return an abstract retur ntype once supported
        let nodes: Vec<_> = self.nodes.iter().filter(|n| n.is_neighbour_of(node.id)).collect();
        nodes.into_iter()
    }

    pub fn nodes(&self) -> ::std::slice::Iter<Node<T>> {
        self.nodes.iter()
    }

    /// Gets the ID of the local node.
    pub fn local_id(&self) -> Id { self.local_id }

    /// Performs sanity checking on the graph.
    pub fn sanity_check(&self) -> Result<(),String> {
        // TODO: make sure there are no redundant nodes
        //       - all node neighbours know  of each other
        //       - no duplicates

        Ok(())
    }
}
