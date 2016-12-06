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

pub struct Iter {}
//TODO Ask Alex, is it bad to not have an Iter for edges? Petgraph does...
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
    /// Add a vertex to a graph, returning an Iter to the inserted vertex.
    /// The lifetime of the Iter is limited to the lifetime of the inserted
    /// vertex.
    #[allow(unused_variables)]
    //pub fn add_vertex(&'a mut self, v: Vertex<V>) -> 'a Iter {
    //TODO Alex, is it even possible to put a lifetime to a nonreference opject
    //as we want to do here? We want to ensure that Iter will not outlive the
    //graph for saftey reasons.
    //One of our ideas for making this work would be to have an Iter contain a
    //reference to an index and insist the the Iter not outlive that reference.
    //We could then return an Iter out of references that do not outlive
    //their graph.
    pub fn add_vertex(&mut self, v: Vertex<V>) -> Iter {
        Iter {}
    }
    /// Add an edge to a graph if there is not currently an edge between those
    /// vertices.  Returns true if successful, and false otherwise.
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, v1: &Iter, v2: &Iter, value: E) -> bool {
        //TODO Ask Alex if this return type is weird (gets back to the "should
        //we have edge Iters?" question).
        true
    }
    /// Returns the old value associated with vertex v and replaces it with the
    /// given value.
    #[allow(unused_variables)]
    pub fn replace_vertex(&mut self, v: &Iter, value: V) -> PhantomData<V> {
        Default::default()
    }
    /// Returns the E which was stored between vertices v1 and v2, leaving the
    /// value in its place, unless there was no such edge, in which case it
    /// lets the value die and returns None.
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, v1: &Iter, v2: &Iter, value: E) ->
        Option<E> {
        None
    }
    /// Delete vertex from a graph, returning a new Graph.
    /// We consume the Graph object so that the compiler prevents existing
    /// Iters from being used after their invalidation.
    #[allow(unused_variables)]
    pub fn delete_vertex(self, v: Iter) -> Self {
        //TODO Alex, is this appropriate? Our plan was to consume the Graph
        //in any method which invalidates iterators so that all iterators from
        //before the call can be statically determined to be invalid after
        //a call to a method that consumes self.
        self
    }
    /// Returns Some(value) associated with the edge between v1 and v2 or None
    /// if there was no such edge.
    #[allow(unused_variables)]
    pub fn delete_edge(&mut self, v1: &Iter, v2: &Iter) -> Option<E> {
        None
    }
    /// Returns a vector of terators neighboring the given vertex.
    #[allow(unused_variables)]
    pub fn get_neighbors(&self, v: &Iter) -> Vec<Iter> {
        Vec::new()
    }
    /// Returns a pair (Graph, Option<Iter>) where the element one is a new
    /// Graph and element two is an Iter to the vertex which results from
    /// contracting the given edge, or None if the edge did not exist.
    #[allow(unused_variables)]
    pub fn contract_edge(self, v1: Iter, v2: Iter) ->
        //('a Self, Option<'a Iter>) {
        //TODO Hi Alex, this is the same issue as the other lifetime question.
        (Self, Option<Iter>) {
            (self, None)
    }
    /// Returns whether or not the given vertices are adjacent.
    #[allow(unused_variables)]
    pub fn adjacent(&self, v1: &Iter, v2: &Iter) -> bool {
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
    pub fn get_laplacian(&self) -> Vec<Vec<isize> > {
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
