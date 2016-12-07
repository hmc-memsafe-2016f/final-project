/// Julien Chien <jchien17@cmc.edu>
///
/// This is a graph data structure that uses adjacency lists
pub struct Graph<T> {
    adj: Vec<Edge>,
    nodes: Vec<Node<T>>,
    num_nodes: usize,
    num_edges: usize
}

/// Edges go from node start_index to node end_index
/// Edges have weight w
pub struct Edge {
    start_index: usize,
    end_index: usize,
    w: usize
}

/// Nodes store data of type T
pub struct Node<T> {
    data: T,
}

/// implementation of my graph
impl<N, E> Graph<N, E> {
    /// creates a new graph
    fn new() -> Self;

    /// returns the number of nodes in the graph
    fn num_nodes(&self) -> usize;

    /// returns the number of edges in the graph
    fn num_edges(&self) -> usize;

    /// adds a node with data T into the graph
    fn add_node(&mut self, data: T) -> usize;

    /// adds an edge of some weight, connecting nodes start to end
    fn add_edge(&mut self, start:usize, end:usize, weight: usize) -> usize;

    /// returns an iterator of Nodes
    fn node_iter(&self) -> Iterator<Node<T>>;

    /// returns an iterator of Edges
    fn edge_iter(&self) -> Iterator<Edge>;

    ///Does Prim's algorithm on the graph
    ///Returns a vetor of a edges that form the minimum spanning tree from Prim's algorithm
    fn prim(&self) -> Vec<Edge>;

    ///Does Kruskal's algorithm on the graph
    ///Returns a vector of edges that form the minimum spanning tree from Kruskal's algorithm
    fn kruskal(&self) -> Vec<Edge>;


}


