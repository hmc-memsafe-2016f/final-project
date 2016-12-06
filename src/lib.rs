/// The Michael Jackson Graph API
///
/// ## Examples
/// ```rust
/// use michael_jackson::Graph;
/// let x = Graph::new();
/// ```

use std::marker::PhantomData;

pub struct Graph<V, E> {
    phantom: PhantomData<(V, E)>,
}
pub struct Vertex<V> {
    value: PhantomData<V>,
}
pub struct Edge<V, E> {
    parent: PhantomData<Vertex<V>>,
    child: PhantomData<Vertex<V>>,
    weight: PhantomData<E>,
}

// Directed/Undirected vs Just undirected
// Add nodes and edges
// Delete nodes and edges
// Modify weights/values
// get Neighbors of a node
// edges can be pattern matched to get endpoints
// edge and vertex contraction (not featured in petgraph)
// test for adjacency
// size()
// get_adjacency_matrix()
// get_laplacian(), number_connected_components()

impl<V, E> Graph<V, E> {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph { phantom: Default::default() }
    }
    /// Construct a graph without data, just for its topology
    #[allow(unused_variables)]
    pub fn new_from_edges(edges: Vec<(u32, u32)>) -> Self {
        Graph { phantom: Default::default() }
    }
    /// Add a vertex to a graph
    #[allow(unused_variables)]
    pub fn add_vertex(v: Vertex<V>) -> () {
    }
    /// Add an edge to a graph
    #[allow(unused_variables)]
    pub fn add_edge(v1: Vertex<V>, v2: Vertex<V>, value: E) -> () {
    }
    /// Delete vertex from a grpah (invalidating all theorems?)
    #[allow(unused_variables)]
    pub fn delete_vertex(v: Vertex<V>) -> () {
    }


}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
*/
