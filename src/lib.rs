/// Graph data type for MemSafety.
///
/// Provides a clean and simple graph API for a graph
/// with arbitrary node and edge types, as well as
/// either directed or undirected graphs.
///
/// Multigraphs are not allowed, and will cause errors.
///
/// All operations involving pre-existing nodes or edges
/// will panic if the assumed node or edge does not exist.
///
/// ## Examples
///
/// ```rust
/// use Graph;
/// let g = Graph::<string, i32, Directed>::new();
/// let a = g.add_node(5);
/// let b = g.add_node(10);
/// g.add_edge(&a, &b);
/// assert_eq!(g.num_nodes() == 2);
/// assert_eq!(g.num_edges() == 1);
/// assert_eq!(g.neighbors(&a) == &b);
/// ```
pub struct Graph<'a, 'b: 'a, NodeType: 'b, EdgeType: 'b, G> {
    // This will be some data members, for example
    adjacency_list: Vector<'a, (NodeType, LinkedList<(NodeIter<'a, 'b, NodeType>, EdgeType)>)>,
    num_nodes: usize,
    num_edges: usize,
}

impl for Graph<'a, NodeType, EdgeType, G> {
  /// Constructs a new graph
  fn new() -> Self;

  /// Returns whether or not the graph is directed.
  fn is_directed(&'a self) -> bool;

  /// Return the number of nodes in the graph in O(1) time.
  fn num_nodes(&'a self) -> usize;
  /// Adds a new node to the graph. Returns an iterator that points to this node.
  fn add_node<'b: 'a>(&'a self, &'b NodeType) -> NodeIter<'a, 'b, NodeType>;
  /// Updates a node in the graph, and returns an iterator that points to this node.
  /// Any pre-existing iterators to this node are invalidated.
  fn update_node<'b: 'a, 'c: 'a>(&'a self, NodeIter<'a, 'b, NodeType>, &'c NodeType) -> NodeIter<'a, 'c, NodeType>;
  /// Removes a node from the graph if it exists, and panics otherwise.
  /// Also removes all edges to/from this node.
  /// All iterators are invalidated.
  fn remove_node<'b: 'a>(&'a self, NodeIter<'a, 'b, NodeType>);

  /// Return the number of edges in the graph in O(1) time.
  fn num_edges(&'a self) -> usize;
  /// Adds a new edge to the graph. Returns an iterator that points to this edge.
  fn add_edge<'b: 'a, 'c: 'a>(&'a self, &'a NodeIter<'a, 'b, NodeType>, &'a NodeIter<'a, 'b, NodeType>, &'c EdgeType) -> EdgeIter<'a, 'c, EdgeType>;
  /// Updates an existing edge in the graph, and returns an edge that points to this edge.
  /// Any pre-existing iterators to this edge are invalidated.
  fn update_edge<'b: 'a, 'c: 'a>(&'a self, EdgeIter<'a, 'b, EdgeType>, &'c EdgeType) -> EdgeIter<'a, 'c, EdgeType>;
  /// Removes an edge from the graph if it exists, and panics otherwise.
  /// All edge iterators are invalidated.
  fn remove_edge<'b: 'a>(&'a self, EdgeIter<'a, 'b, EdgeType>);

  /// Returns an iterator of the neighbors of a node
  fn neighbors<'b: 'a>(&'a self, &'a NodeIter<'a, 'b, NodeType>) -> NodeIter<'a, 'b, NodeType>;
  /// Returns an iterator of the neighbors of a node based on a given direction.
  fn neighbors_direction<'b: 'a>(&'a self, &'a NodeIter<'a, 'b, NodeType>, Direction) -> NodeIter<'a, 'b, NodeType>;
  /// Returns the number of neighbors of a node
  fn degree<'b: 'a>(&'a self, &'a NodeIter<'a, 'b, NodeType>) -> usize;
  /// Returns the number of neighbors of a node in a given direction
  fn degree<'b: 'a>(&'a self, &'a NodeIter<'a, 'b, NodeType>, Direction) -> usize;

  /// Returns the endpoints of an edge
  fn endpoints<'b: 'a>(&'a self, &'a EdgeIter<'a, 'b, EdgeType>) -> (NodeIter<'a, 'b, NodeType>, NodeIter<'a, 'b, NodeType>);
}

/// Represents the direction of an edge
pub enum Direction {
    /// Edges coming into a given node
    Incoming,
    /// Edges leaving a given node
    Outgoing,
    /// Any edge connected to a node in an undirected graph
    Undirected
}

/// Represents the type of a graph, Directed or Undirected
pub enum GraphType {
    Directed,
    Undirected
}

use std::marker::PhantomData;

/// Iterator for edges in a graph
pub struct EdgeIter<'a, 'b: 'a, T: 'b> {
    value: &'b T,
    a: PhantomData<&'a T>
}

/// Iterator for nodes in a graph
pub struct NodeIter<'a, 'b: 'a, T: 'b> {
    value: &'b T,
    a: PhantomData<&'a T>
}

/// Finds the shortest path to all other nodes in a graph
pub fn dijkstra<'a, Node, Edge, G>(&Graph<'a, Node, Edge, G>, NodeIter<'a, Node>) -> LinkedList<(usize, NodeIter<'a, Node>)>;

/// Determines if two graphs are isomorphically equivalent
pub fn vf2<'a, 'b, Node, Edge, G1, G2, NodeF, EdgeF>(
    &Graph<'a, Node, Edge, G1>, &Graph<'b, Node, Edge, G2>,
    NodeF, EdgeF) -> bool
  where NodeF: Fn(Node, Node) -> bool,
        EdgeF: Fn(Node, Node) -> bool;
