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
    weight: PhantomData<E>,
    parent: PhantomData<V>,
    child: PhantomData<V>,
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
    pub fn new_from_edges(edges: Vec<(u32, u32)>) -> Self {
        Graph { phantom: Default::default() }
    }
    /// Add a vertex to a graph
    pub fn add_vertex(v: Vertex<V>) -> Self {
        Graph { phantom: Default::default() }
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
