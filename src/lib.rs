// lib.rs

use std::collections::{HashMap,LinkedList};
use std::rc::Rc;

/// A pairing heap for implementing Djikstra's and Prim's algorithms.
///
/// For information on pairing heaps see https://en.wikipedia.org/wiki/Pairing_heap.
///
/// XXX: this will need some work, see https://github.com/contain-rs/discuss/issues/11
/// for details.
struct PairingHeap<Key, Value> {
    key: Key,
    value: Value,
    children: LinkedList<PairingHeap<Key, Value>>,
}

impl<Key, Value> PairingHeap<Key, Value> {
    /// Create a new, empty `PairingHeap`.
    ///
    /// Takes *O(1)* time.
    fn new() -> Self {
        unimplemented!();
    }

    /// Insert a new weight and value. This is the
    /// sketchy part of this API in Rust: insert returns a *pointer*
    /// into the heap
    /// 
    /// Takes *O(1)* time.
    fn insert(&mut self, key: Key, val: Value) -> *const PairingHeap<Key, Value> {
        unimplemented!();
    }

    /// Remove the minimum key and value, if it exists.
    ///
    /// Takes *O(log n)* amortized time.
    fn delete_min(&mut self) -> Option<(Key, Value)> {
        unimplemented!();
    }

    /// Get references to the minimum key and value.
    ///
    /// Takes *O(1)* time.
    fn find_min(&self) -> Option<(&Key, &Value)> {
        unimplemented!();
    }

    /// join two PairingHeaps (this is mostly an implementation detail)
    ///
    /// Takes *O(1)* time.
    fn meld(&mut self, other: Self) {
        unimplemented!();
    }

    /// decrease the key associated with a particular node. This function
    /// is `unsafe` because we have no way to validate the pointer in a reasonable
    /// amount of time
    /// 
    /// Takes *O(log 1)* ammortized time.
    unsafe fn decrease_key(&mut self, node: *const PairingHeap<Key, Value>, new_val: Key) {
        unimplemented!();
    }
}

/// A small, type-generic graph API that can compute the length of the shortest path between
/// verticies and the minimum spanning tree. 
/// 
/// The Graph object *does not* own its data, it merely borrows it from the owner.
///
/// The object caches shortest path lengths and the minimum spanning tree.
///
/// The Graph only needs a list of vertices. Adjacency is determined by a user-supplied functor
/// that takes a vertex and returns a `Vec` of adjacent vertices and their weights. (XXX: should
/// return an iterator). This allows this data structure to represent graphs where the edges are
/// implicit (i.e. the Peteren Graph where the edges can be determined from the vertex values)
pub struct Graph<'a, Vertex: 'a, Weight, AdjF> {

    /// user-supplied container of verticies
    vertices: &'a Vec<Rc<Vertex>>,

    /// user-supplied function to get vertices adjacent to a vertex
    /// returns a container of pairs of the form (vertex, weight)
    ///
    /// XXX: this function should really return an iterator; we don't want
    /// to copy all the vertices adjacent to a vertex every time we want
    /// to know them; we just want to iterate through them.
    adj: AdjF,

    /// cache of shortest paths from given vertices
    /// Find a weight as e.g. sp_cache.get(src).unwrap().get(dst).uwrap()
    sp_cache: HashMap<Rc<Vertex>, HashMap<Rc<Vertex>, Weight>>,

    /// cache of spanning tree, represented as adjacency list
    st_cache: Option<Vec<Vec<Rc<Vertex>>>>,
}

impl<'a, Vertex, Weight, AdjF: FnMut(&Vertex) -> Vec<(Rc<Vertex>, Weight)>>
    Graph<'a, Vertex, Weight, AdjF> {

    /// Gonstruct a new, empty graph given a vertex list and an adjacency functor.
    pub fn new(vertices: &'a Vec<Rc<Vertex>>, adj: AdjF)
               -> Graph<'a, Vertex, Weight, AdjF> {
        unimplemented!();
    }

    /// Given a source and destination vertex, determine the length of the shortest
    /// path between them.
    ///
    /// If this function has been previously called with the same `src` vertex, the
    /// runtime is *O(1)*, otherwise this has the same runtime as Djikstra's algorithm,
    /// i.e. *O(E + V log V)*.
    pub fn shortest_path_len(src: &Vertex, dst: &Vertex) -> Weight {
        unimplemented!();
    }

    /// Return a minimum spanning tree represented as an adjacency list.
    ///
    /// If this function has been called before, the runtime is *O(V + E)*, i.e. the
    /// time it takes to make a copy of the spanning tree, otherwise the runtime is
    /// the that of Prim's algorithm, i.e. *O(E + V log V)*.
    pub fn spanning_tree() -> Vec<Vec<Rc<Vertex>>> {
        unimplemented!();
    }
}
