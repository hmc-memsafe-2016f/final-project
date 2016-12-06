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

struct iterator {}
//TODO Ask Alex, is it bad to not have an iterator for edges? Petgraph does...
//Not doing so means replace_edge is not constant time.

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
    /// Add a vertex to a graph, returning an iterator to the inserted vertex.
    /// The lifetime of the iterator is limited to the lifetime of the inserted
    /// vertex.
    #[allow(unused_variables)]
    pub fn add_vertex(&'a mut self, v: Vertex<V>) -> 'a iterator {
        //TODO return an iterator???profit
    }
    /// Add an edge to a graph if there is not currently an edge between those
    /// vertices.  Returns true if successful, and false otherwise.
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, v1: iterator, v2: iterator, value: E) -> bool {
        //TODO Ask Alex if this return type is weird (gets back to the "should
        //we have edge iterators?" question.
        true
    }
    /// Returns the old value associated with vertex v and replaces it with the
    /// given value.
    #[allow(unused_variables)]
    pub fn replace_vertex(&mut self, value: V, v: iterator) -> V {
        Default::default()
    }
    /// Returns the E which was stored between vertices v1 and v2, leaving the
    /// value in its place, unless there was no such edge, in which case it
    /// lets the value die and returns None.
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, value: E, v1: iterator, v2: iterator) ->
        Option<E> {
        None
    }
    /// Delete vertex from a graph, returning a new Graph.
    /// We consume the Graph object so that the compiler prevents existing
    /// iterators from being used after their invalidation.
    #[allow(unused_variables)]
    pub fn delete_vertex(self, v: Vertex<V>) -> Self {
    }
    /// Returns Some(value) associated with the edge between v1 and v2 or None
    /// if there was no such edge.
    #[allow(unused_variables)]
    pub fn delete_edge(&mut self, v1: Vertex<V>, v2: Vertex<V>) -> Option<E> {
        None
    }
    /// Returns a vector of terators neighboring the given vertex.
    #[allow(unused_variables)]
    pub fn get_neighbors(&self, v: iterator) -> Vec<iterator> {
        Vec::new()
    }
    /// Returns a pair (Graph, Option<iterator>) where the element one is a new
    /// Graph and element two is an iterator to the vertex which results from
    /// contracting the given edge, or None if the edge did not exist.
    #[allow(unused_variables)]
    pub fn contract_edge(self, v1: iterator, v2: iterator) ->
        ('a Self, Option<'a iterator>) {
            (self, None)
    }
    /// Returns whether or not the given vertices are adjacent.
    #[allow(unused_variables)]
    pub fn contract_edge(&self, v1: iterator, v2: iterator) -> bool {
        true
    }
    /// Returns the number of vertices in the graph.
    #[allow(unused_variables)]
    pub fn num_vertices(&self) -> usize {
        0
    }
    /// Returns the number of edges in the graph.
    #[allow(unused_variables)]
    pub fn num_edges(&self) -> usize {
        0
    }
    /// Returns the adjacency matrix for the given graph.
    #[allow(unused_variables)]
    pub fn get_adjacency_matrix(&self) -> Vec<Vec<&E> > {
        //TODO Ask Alex if Vec<Vec>> is a reasonable matrix representation.
        Vec::new()
    }
    /// Returns the Laplacian matrix for the given graph.
    #[allow(unused_variables)]
    pub fn get_adjacency_matrix(&self) -> Vec<Vec<isize> > {
        Vec::new()
    }
    /// Returns the number of connected components in the graph.
    #[allow(unused_variables)]
    pub fn num_components(&self) -> usize {
        1
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
