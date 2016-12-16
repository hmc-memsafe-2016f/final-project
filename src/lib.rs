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
use std::sync::RwLock;

/// A data structure which represents a mathematical graph.
/// It is implemented as an adjacency list (a vector of Linked Lists) together
/// with a Vector of vertices.
pub struct Graph<V, E> {
    adj_list: RwLock<Vec<LinkedList<Edge<E>>>>,
    vertices: RwLock<Vec<Vertex<V>>>,
}

/// A vertex, can be inserted into a Graph and holds data of arbitrary type.
pub struct Vertex<V> {
    contents: V,
}

/// A public struct in the Graph's adjacency list which keeps indices to
/// both endpoints and the data associated with the edge.
#[derive(Debug)]
pub struct Edge<E> {
    pub parent: usize,
    pub child: usize,
    pub weight: E,
}

/// Iterator struct which keeps track of the location of a vertex within the Graph
/// struct. Can be used to iterate over vertices in an arbitrary order.
pub struct VRef<'a, V: 'a, E: 'a> {
    index: usize,
    graph: &'a Graph<V, E>,
}

impl<'a, V, E> Clone for VRef<'a, V, E> {
    fn clone(&self) -> Self {
        VRef{ index: self.index, graph: self.graph }
    }
}

impl<'a, V, E> Copy for VRef<'a, V, E> {
}

/// Returns an EIter which iterates over the edges of a vertex
impl<'a, V, E> VRef<'a, V, E> {
    pub fn edges(&self) -> EIter<'a, E> {
        unimplemented!()
        //let adj_list = self.graph.adj_list.read().unwrap();
        //EIter { iter: adj_list[self.index].iter() }
    }
}

impl<'a, V, E> Deref for VRef<'a, V, E> {
    type Target = V;
    fn deref(&self) -> &V {
        let vertices = self.graph.vertices.read().unwrap();
        unsafe {
            let vertex_ptr = &vertices[self.index].contents as *const V;
            vertex_ptr.as_ref().unwrap()
        }
    }
}

pub struct VIter<'a, V: 'a, E: 'a> {
    r: VRef<'a, V, E>,
}

impl<'a, V, E: Clone + Copy + PartialEq> Iterator for VIter<'a, V, E> {
    type Item = VRef<'a, V, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.r.index < Graph::num_vertices(self.r.graph) {
            let old_ref = VRef{ index: self.r.index, graph: self.r.graph };
            self.r.index = self.r.index + 1;
            Some(old_ref)
        }
        else {
            None
        }
    }
}

pub struct ERef<'a, E: 'a> {
    r: &'a Edge<E>
}

impl<'a, E> Deref for ERef<'a, E> {
    type Target = Edge<E>;
    fn deref(&self) -> &Edge<E> {
        &(self.r)
    }
}

pub struct EIter<'a, E: 'a> {
    iter: linked_list::Iter<'a, Edge<E>>
}

impl<'a, E> Iterator for EIter<'a, E> {
    type Item = ERef<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge_ref| ERef{ r: edge_ref })
    }
}


// Some functions require types V and E to have default values
impl<'a, V: Hash + Eq + Copy + Clone, E: Default + Copy + Clone + PartialEq> Graph<V, E> {
    /// Returns an iterator that ranges over all vertices in arbitrary order.
    pub fn vertices(&'a self) -> VIter<'a, V, E> {
        VIter{ r: VRef{ index: 0, graph: &self } }
    }

    /// Construct a graph without data, with default values for V and E
    /// and populates the given iters vector with iterators to the added vectors
    /// in the order they were encountered. Clears the given vector before
    /// populating.
    #[allow(unused_variables)]
    pub fn extend_with_edges(&'a self, edges: &Vec<(V, V)>) -> Vec<VRef<'a, V, E>> {
        let mut vrefs : Vec<VRef<'a, V, E>> = Vec::new();
        let mut ref_map = HashMap::new();

        for vref in self.vertices() {
            ref_map.insert(*vref, vref);
        }

        for &(u, v) in edges {
            if !ref_map.contains_key(&u) {
                let vref : VRef<'a, V, E> = self.add_vertex(u);
                vrefs.push(vref);
                ref_map.insert(u, vref);
            }
            if !ref_map.contains_key(&v) {
                let vref = self.add_vertex(v);
                vrefs.push(vref);
                ref_map.insert(v, vref);
            }
            self.add_directed_edge(ref_map.get(&u).unwrap(), ref_map.get(&v).unwrap(), E::default());
        };
        vrefs
    }
}

impl<V, E: Clone + Copy + PartialEq> Graph<V, E> {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph{ adj_list: RwLock::new(Vec::new()) , vertices: RwLock::new(Vec::new()) }
    }

    /// Add a vertex to a graph, returning an VRef to the inserted vertex.
    /// The lifetime of the VRef is limited to the lifetime of the inserted
    /// vertex.
    #[allow(unused_variables)]
    pub fn add_vertex(&self, v: V) -> VRef<V, E> {
        self.vertices.write().unwrap().push(Vertex{ contents: v } );
        self.adj_list.write().unwrap().push(LinkedList::new());
        VRef { index: self.vertices.read().unwrap().len(), graph: &self }
    }

    /// Add a directed edge to the graph from v1 to v2 with weight `value`.
    /// Returns an ERef to the added edge.
    pub fn add_directed_edge<'a>(&'a self, v1: &VRef<V, E>, v2: &VRef<V, E>, value: E)
            -> ERef<'a, E> {
        {
        let mut adj_list = self.adj_list.write().unwrap();
        adj_list[v1.index].push_front(Edge{ parent: v1.index, child: v2.index, weight: value });
        }
        let adj_list = self.adj_list.read().unwrap();
        unsafe {
            let edge_ref = adj_list[v1.index].front().unwrap() as *const Edge<E>;
            ERef{ r: edge_ref.as_ref().unwrap() }
        }
    }

    /// Adds an undirected edge to the graph between v1 and v2.
    /// Returns a tuple of ERefs to the added edges.
    pub fn add_undirected_edge(&self, v1: &VRef<V, E>, v2: &VRef<V, E>, value: E)
            -> (ERef<E>, ERef<E>) {
        let ref1 = Graph::add_directed_edge(&self, v1, v2, value);
        let ref2 = Graph::add_directed_edge(&self, v2, v1, value);
        (ref1, ref2)
    }

    /// Returns the old value associated with vertex v and replaces it with the
    /// given value.
    #[allow(unused_variables)]
    pub fn replace_vertex(&self, v: &VRef<V, E>, value: V) -> PhantomData<V> {
        Default::default()
    }

    /// Returns a Vec<VRef> of those neighboring the given vertex.
    #[allow(unused_variables)]
    pub fn get_neighbors(&self, v: &VRef<V, E>) -> Vec<VRef<V, E>> {
        Vec::new()
    }

    /// Returns whether or not there is a directed edge in both directions
    /// with values of E which are equal
    #[allow(unused_variables)]
    pub fn adjacent_undirected(&self, v1: &VRef<V, E>, v2: &VRef<V, E>) -> bool {
        let adj_list = self.adj_list.read().unwrap();
        for edge1 in &adj_list[v1.index] {
            if edge1.child == v2.index {
                for edge2 in &adj_list[v2.index] {
                    if edge2.child == v1.index {
                        if edge1.weight == edge2.weight {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    /// Returns the number of vertices in the graph.
    #[allow(unused_variables)]
    pub fn num_vertices(&self) -> usize {
        self.vertices.read().unwrap().len()
    }

    /// Returns the number of edges in the graph.
    #[allow(unused_variables)]
    pub fn num_edges(&self) -> usize {
        let mut edges = 0;
        for list in self.adj_list.read().unwrap().iter() {
            edges += list.len();
        }
        edges
    }

    /// Returns the adjacency matrix for the given graph.
    #[allow(unused_variables)]
    pub fn get_adjacency_matrix(&self) -> Vec<usize> {
        let size = Graph::num_vertices(self);
        let at = |r: usize, c: usize| r * size + c;
        let mut matrix = vec![0; size * size];
        //for eIter in Graph::edges {
        //}
        matrix
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
