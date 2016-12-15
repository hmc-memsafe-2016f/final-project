/// The Michael Jackson Graph API
///
/// ## Examples
/// ```rust
/// use michael_jackson::Graph;
/// let x = Graph::new();
/// ```

use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::linked_list;
use std::marker::PhantomData;
use std::ops::Deref;
use std::hash::Hash;
use std::cmp::Eq;

/// A data structure which represents a mathematical graph.
/// It is implemented as an adjacency list (a vector of Linked Lists) together
/// with a Vector of vertices.
pub struct Graph<V, E> {
    adj_list: Vec<LinkedList<Edge<E>>>,
    vertices: Vec<Vertex<V>>,
}

/// A vertex, can be inserted into a Graph and holds data of arbitrary type.
pub struct Vertex<V> {
    contents: V,
}

/// A private struct in the Graph's adjacency list which keeps indices to
/// both endpoints and the data associated with the edge.
struct Edge<E> {
    parent: usize,
    child: usize,
    weight: E,
}

/// Iterator struct which keeps track of the location of a vertex within the Graph
/// struct. Can be used to iterate over vertices in an arbitrary order.
pub struct vIter<'a, V: 'a, E: 'a> {
    index: usize,
    graph: &'a Graph<V, E>,
}
impl<'a, V, E> Deref for vIter<'a, V, E> {
    type Target = V;
    fn deref(&self) -> &V {
        unimplemented!()
    }
}

pub struct eIter<'a, E: 'a> {
    iter: linked_list::Iter<'a, Edge<E>>
}
impl<'a, E> Deref for eIter<'a, E> {
    type Target = E;
    fn deref(&self) -> &E {
        unimplemented!()
    }
}

// Some functions require types V and E to have default values
impl<V: Hash + Eq, E: Default> Graph<V, E> {
    /// Construct a graph without data, with default values for V and E
    /// and populates the given iters vector with iterators to the added vectors
    /// in the order they were encountered. Clears the given vector before
    /// populating.
    #[allow(unused_variables)]
    pub fn new_from_edges_populate_iters(edges: &Vec<(V, V)>,
                                         iters: &mut Vec<vIter<V,E>>) -> Self {
        let g = Graph::new();
        iters.clear();
        let mut iter_map = HashMap::new();
        for (u, v) in edges.as_ref() {
                if !iter_map.contains_key(u) {
                    let iter : vIter<V,E> = g.add_vertex(*u);
                    iters.push(iter);
                    iter_map.insert(u, iters.last().unwrap());
                }
                if !iter_map.contains_key(v) {
                    let iter : vIter<V,E> = g.add_vertex(*v);
                    iter_map.insert(v, iters.last().unwrap());
                    iters.push(iter);
                }
            g.add_edge(iter_map.get(u).unwrap(), iter_map.get(v).unwrap(), E::default());
        }
        g
    }
    /// Construct a graph without data, with default values for V and E
    #[allow(unused_variables)]
    pub fn new_from_edges(edges: &Vec<(V, V)>) -> Self {
        let mut v = Vec::new();
        Graph::new_from_edges_populate_iters(edges, &mut v);
    }
}

impl<V, E> Graph<V, E> {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph{ adj_list: Vec::new(), vertices: Vec::new() }
    }

    /// Add a vertex to a graph, returning an vIter to the inserted vertex.
    /// The lifetime of the vIter is limited to the lifetime of the inserted
    /// vertex.
    #[allow(unused_variables)]
    //pub fn add_vertex(&'a mut self, v: Vertex<V>) -> 'a vIter<V, E> {
    //TODO Alex, is it even possible to put a lifetime to a nonreference opject
    //as we want to do here? We want to ensure that vIter will not outlive the
    //graph for saftey reasons.
    //One of our ideas for making this work would be to have an vIter contain a
    //reference to an index and insist the the vIter not outlive that reference.
    //We could then return an vIter out of references that do not outlive
    //their graph.
    pub fn add_vertex(&self, v: V) -> vIter<V, E> {
        unimplemented!()
        //vIter<V, E> { index: }
    }

    /// Add an edge to a graph if there is not currently an edge between those
    /// vertices.  Returns true if successful, and false otherwise.
    #[allow(unused_variables)]
    pub fn add_edge(&self, v1: &vIter<V, E>, v2: &vIter<V, E>, value: E) ->
        Option<eIter<E>> {
        //TODO Ask Alex if this return type is weird (gets back to the "should
        //we have edge vIters?" question).
        unimplemented!()
    }

    /// Returns the old value associated with vertex v and replaces it with the
    /// given value.
    #[allow(unused_variables)]
    pub fn replace_vertex(&self, v: &vIter<V, E>, value: V) -> PhantomData<V> {
        Default::default()
    }

    /// Returns the E which was stored between vertices v1 and v2, leaving the
    /// value in its place, unless there was no such edge, in which case it
    /// lets the value die and returns None.
    #[allow(unused_variables)]
    pub fn replace_edge(&self, v1: &vIter<V, E>, v2: &vIter<V, E>, value: E) ->
        Option<E> {
        None
    }

    /// Returns a vector of terators neighboring the given vertex.
    #[allow(unused_variables)]
    pub fn get_neighbors(&self, v: &vIter<V, E>) -> Vec<vIter<V, E>> {
        Vec::new()
    }

    /// Returns whether or not the given vertices are adjacent.
    #[allow(unused_variables)]
    pub fn adjacent(&self, v1: &vIter<V, E>, v2: &vIter<V, E>) -> bool {
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
    pub fn get_adjacency_matrix(&self) -> Vec<usize> {
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
