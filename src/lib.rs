// lib.rs

use std::collections::HashMap;

/// a private module containing a pairing min heap
pub mod pairing_heap {

    use std::sync::atomic::{AtomicUsize,Ordering,ATOMIC_USIZE_INIT};
    use std::cmp::{Ord,Eq};
    use std::ptr::null_mut;
    use std::mem;
    use std::fmt;
    
    struct PairingHeapNode<Key, Value> {
        key: Key,
        value: Value,

        /// The following 4 fields kinda suck; it would be nice if we could use some standard
        /// containers here, but we need precise control over where these objects live (i.e.
        /// they need to never move), so we have to manage all the memory ourselves. Sigh.
        
        /// the parent node in the heap
        parent: *mut PairingHeapNode<Key, Value>,

        /// the first node in the linked list of children
        child: *mut PairingHeapNode<Key, Value>,

        /// this node's siblings (doubly linked list)
        prev: *mut PairingHeapNode<Key, Value>,
        next: *mut PairingHeapNode<Key, Value>,
    }
    
    impl <Key, Value> PairingHeapNode<Key, Value> where
        Key: Ord + Eq + Copy
    {
            
        /// Create a new node with 
        fn new(k: Key, v: Value) -> Self {
            PairingHeapNode{key: k,
                 value: v,
                 parent: null_mut(),
                 child: null_mut(),
                 prev: null_mut(),
                 next: null_mut()}
        }

        /// orphan a subtree from the heap it's in
        unsafe fn orphan(&mut self) {
            if !self.prev.is_null() {
                // fix up the sibling list where the child used to live
                (*self.prev).next = self.next;
            } else if !self.parent.is_null() {
                // we were the first node in our parent's child list, so fix up our parent's child ptr
                assert!((*self.parent).child == (self as *mut _));
                (*self.parent).child = self.next;
            }

            if self.next != null_mut() {
                (*self.next).prev = self.prev;
            }

            self.prev = null_mut();
            self.next = null_mut();
            self.parent = null_mut();
        }

        /// given a node (subtree), make it a child node of self while carefully removing it
        /// from wherever it used to live in the tree.
        unsafe fn add_child(&mut self, _child: *mut PairingHeapNode<Key, Value>) {
            assert!((self as *mut _) != _child);
            assert!(!_child.is_null());
            assert!(!((*_child).key < self.key));

            let child = &mut *_child;

            // put the child in our child node list
            child.next = self.child;
            child.parent = self;
            if !child.next.is_null() {
                (*child.next).prev = _child;
            }
            self.child = _child;
        }

        /// Merge a node into the current subtree, preserving the heap ordering
        unsafe fn merge(&mut self, other: *mut PairingHeapNode<Key, Value>) -> *mut PairingHeapNode<Key, Value> {
            self.orphan();

            if other.is_null() {
                self as *mut _
            } else {
                let other_ref = &mut *other;
                other_ref.orphan();

                if self.key < other_ref.key {
                    self.add_child(other);
                    self as *mut _
                } else {
                    other_ref.add_child(self as *mut _);
                    other
                }
            }
        }

        /// fix up the heap after deleting an element
        unsafe fn merge_pairs(node: *mut PairingHeapNode<Key, Value>) -> *mut PairingHeapNode<Key, Value> {
            if node.is_null() || (*node).next.is_null() {
                node
            } else {
                let next = (*node).next;
                let next_next = (*next).next;
                let ret = (*node).merge(next);
                (*ret).merge(Self::merge_pairs(next_next))
            }
        }
    }

    impl<Key, Value> Drop for PairingHeapNode<Key, Value> {
        fn drop(&mut self) {
            unsafe {
                let mut child = self.child;
                while !child.is_null() {
                    let next = (*child).next;
                    // recurse
                    Box::from_raw(child);
                    child = next;
                }
            }
        }
    }

    impl<Key: fmt::Display,Value> fmt::Display for PairingHeapNode<Key, Value> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(")
                .and_then(|()| self.key.fmt(f))
                .and_then(|()|
                    if !self.child.is_null() {
                        write!(f, ", ")
                    } else {
                        Ok(())
                    })
                .and_then(|()| {
                    let mut node = self.child;
                    if !node.is_null() {
                        try!(write!(f, "("));
                    } else {
                        return Ok(())
                    }
                    while !node.is_null() {
                        try!(unsafe {(*node).fmt(f)});
                        let next = unsafe {(*node).next};
                        // don't write another comma unless there are more nodes
                        if !next.is_null() {
                            try!(write!(f, ", "));
                        }
                        node = next;
                    }
                    write!(f, ")")
                })
                .and_then(|()| write!(f, ")"))
        }
    }

    /// An opaque handle to a pairing heap node. These are passed out by the PairingHeap API when
    /// a pointer would otherwise be returned.
    ///
    /// This makes the API safe because the pointer data member is not `pub` so the user can not
    /// touch it.
    pub struct PHNodeHandle<Key, Value> {
        ptr: *mut PairingHeapNode<Key, Value>,

        // see the comment above `pairing_heap_id_generator`
        id: usize,
    }

    impl<Key, Value> PHNodeHandle<Key, Value> {

        fn new(heap: &PairingHeap<Key, Value>, n: *mut PairingHeapNode<Key, Value>) -> Self {
            PHNodeHandle{ptr: n, id: heap.id}
        }
    }

    /// To make our `PairingHeap` API safe, we need a way to associate a `PHNodeHadle` with a specific
    /// heap. We could keep a pointer to the pairing heap instance, but that breaks as soon as we
    /// move the `PairingHeap`. We could also disallow moving of `PairingHeaps`, but that would be
    /// an silly limitation. Thus we need some sort of unique identifier for each heap.
    ///
    /// The simplest way to do that is to use a global counter to generate IDs, i.e. each time we
    /// construct a new heap we:
    ///     1. increment the counter
    ///     2. assign the old value of the counter to `heap.id`
    ///
    /// To guarentee thread safety we use an atomic (also because rust won't allow us to have
    /// non-atonic non-const globals), and we accomplish both steps at once with a `fetch_add`
    ///
    /// Note that there is still one problem with this approach: we could potentaily overflow this
    /// counter and start re-using IDs if someone is using a metric shitload of `PairingHeaps`.
    /// This is where we say "this is a final project for a 1 credit class" and just `panic!()`
    /// if we ever overflow.
    static PAIRING_HEAP_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

    /// A pairing heap for implementing Djikstra's and Prim's algorithms.
    ///
    /// For information on pairing heaps see https://en.wikipedia.org/wiki/Pairing_heap.
    ///
    /// XXX: this will need some work, see https://github.com/contain-rs/discuss/issues/11
    /// for details.
    pub struct PairingHeap<Key, Value> {
        root: *mut PairingHeapNode<Key, Value>,

        // see the comment above PAIRING_HEAP_ID_GENERATOR
        id: usize,
    }    

    impl<Key: Ord + Eq + Copy, Value> PairingHeap<Key, Value> {

        fn get_new_id() -> usize {
            let id = PAIRING_HEAP_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
            if id == usize::max_value() {
                panic!("PairingHeap: overflowed PAIRING_HEAP_ID_GENERATOR!");
            }
            id
        }

        /// Create a new, empty `PairingHeap`.
        ///
        /// Takes *O(1)* time.
        pub fn new() -> Self {
            PairingHeap{root: null_mut(), id: Self::get_new_id() }
        }

        /// creates a new empty `ParingHeap` with a single key-value pair
        pub fn new_with_kv(k: Key, v: Value) -> Self {
            PairingHeap{root: Box::into_raw(Box::new(PairingHeapNode::new(k, v))), id: Self::get_new_id() }
        }

        /// Insert a new weight and value and return a handle to the inserted node.
        ///
        /// Takes *O(1)* time.
        pub fn insert(&mut self, key: Key, val: Value) -> PHNodeHandle<Key, Value> {
            let node = Box::into_raw(Box::new(PairingHeapNode::new(key, val)));
            self.root = unsafe {(*node).merge(self.root)};
            PHNodeHandle::new(self, node)
        }

        /// Remove the minimum key and value, if it exists.
        ///
        /// Takes *O(log n)* amortized time.
        pub fn delete_min(&mut self) -> Option<(Key, Value)> {
            if self.root.is_null() {
                return None;
            } else {
                unsafe {
                    let mut root = Box::from_raw(self.root);

                    // remove the key and value from the node and make sure their destructors don't
                    // run when `root` goes out of scope
                    let key = mem::replace(&mut root.key, mem::uninitialized());
                    let value = mem::replace(&mut root.value, mem::uninitialized());
                    mem::forget(&root.key);
                    mem::forget(&root.value);

                    // we're about to remove the root node, so make sure none of its children
                    // point to it anymore
                    let mut child = root.child;
                    while !child.is_null() {
                        (*child).parent = null_mut();
                        child = (*child).next;
                    }

                    // grab the first child again, then null the pointer in the root node so the
                    // remainder of the heap doesn't get `drop`'d when `root` goes out of scope
                    child = root.child;
                    root.child = null_mut();

                    // clean up the rest of the tree
                    self.root = PairingHeapNode::merge_pairs(child);

                    Some((key, value))
                }
            }
        }

        /// Get references to the minimum key and value.
        ///
        /// Takes *O(1)* time.
        pub fn find_min(&self) -> Option<(&Key, &Value)> {
            if self.root.is_null() {
                None
            } else {
                let node = unsafe {&*self.root};
                Some((&node.key, &node.value))
            }
        }

        /// join two PairingHeaps
        ///
        /// Takes *O(1)* time.
        pub fn merge(&mut self, other: Self) {
            if !self.root.is_null() {
                self.root = unsafe { (*self.root).merge(other.root) };
            } else {
                self.root = other.root;
            }
            mem::forget(other);
        }

        /// update the key associated with a particular node with a smaller key. If you try
        /// to pass a handle that came from a different heap, this function panics. It also
        /// panics if `new_key` is not smaller than or equal to the previous key.
        /// 
        /// Takes *O(log 1)* ammortized time.
        pub fn update_key(&mut self, handle: &PHNodeHandle<Key, Value>, new_key: Key) {
            if handle.id != self.id {
                panic!("PairingHeap::decrease_key: got handle from different heap!");
            }

            let node = handle.ptr;

            // if this assert trips, it's a programmer erorr on my part, not the user's
            assert!(!node.is_null());

            let node_ref = unsafe {&mut *node};
            assert!(new_key <= node_ref.key);
            node_ref.key = new_key;

            if self.root != node {
                // Node::merge handles removing the node from the heap before merging it
                self.root = unsafe { (*self.root).merge(node) };
            }
        }

        pub fn is_empty(&self) -> bool {
            self.root.is_null()
        }
    }

    impl<Key, Value> Drop for PairingHeap<Key, Value> {
        fn drop(&mut self) {
            if !self.root.is_null() {
                unsafe { Box::from_raw(self.root) };
            }
        }
    }

    impl<Key: fmt::Display,Value> fmt::Display for PairingHeap<Key, Value> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if !self.root.is_null() {
                unsafe {(*self.root).fmt(f)}
            } else {
                Ok(())
            }
        }
    }
}
    
/// A small, type-generic graph API that can compute the length of the shortest path between
/// verticies and the minimum spanning tree. 
/// 
/// The Graph object *does not* own its data, it merely borrows it from the owner.
///
/// The object caches shortest path lengths and the minimum spanning tree.
///
/// The Graph only needs a vertex iterator. Adjacency is determined by a user-supplied functor
/// that takes a vertex and returns a an iterator of adjacent vertices and their weights. This
/// allows this data structure to represent graphs where the edges are
/// implicit (i.e. the Peteren Graph where the edges can be determined from the vertex values)
pub struct Graph<Vertex, VertIter, AdjF> {

    /// user-supplied iterator over a list of vertices
    vertices: VertIter,

    /// user-supplied function to get vertices adjacent to a vertex
    /// returns an iterator of pairs of the form (vertex, weight)
    adj: AdjF,

    /// cache of shortest paths from given vertices
    sp_cache: HashMap<Vertex, HashMap<Vertex, usize>>,

    /// cache of spanning tree, represented as adjacency list
    st_cache: Option<Vec<Vec<Vertex>>>,
}

use std::cmp::Eq;
use std::hash::Hash;

// fun Rust generics facts:
// Note that the "impl" block below has 5 parameters, but the Graph itself only has 4.
// In general, the parameters of an impl block seem to be completely unrelated to the
// parameters of the struct you're writing the block for.
//
// Why in the world would we want to do this?
//
// Note the AdjF type and the AdjIter type (the AdjIter type is the only type that is in the
// impl parameter list but absent from the Graph parameter list). The AdjIter type
// represents the *return type* of AdjF. We want to constrain the return type as implementing
// `Iterator<Item=(&'a Vertex, usize)>`, and we need the helper type AdjIter for this.
impl<'a, Vertex, VertIter, AdjIter, AdjF> Graph<Vertex, VertIter, AdjF> where
    Vertex: Eq + Hash + Copy + std::fmt::Debug,
    VertIter: Iterator<Item=Vertex> + Clone,
    AdjIter: Iterator<Item=(Vertex, usize)>,
    AdjF: FnMut(Vertex) -> AdjIter,
{
    /// Gonstruct a new, empty graph given a vertex list and an adjacency functor.
    pub fn new(iter: VertIter, f: AdjF) -> Self {
        Graph{vertices: iter, adj: f, sp_cache: HashMap::new(), st_cache: None}
    }

    /// Given a source and destination vertex, determine the length of the shortest
    /// path between them.
    ///
    /// If this function has been previously called with the same `src` vertex, the
    /// runtime is *O(1)*, otherwise this has the same runtime as Djikstra's algorithm,
    /// i.e. *O(E + V log V)*.
    ///
    /// See https://en.wikipedia.org/wiki/Dijkstra's_algorithm#Pseudocode
    ///
    /// NB: this implementation kinda sucks because we're using HashMaps like they're
    /// going out of style, but this is the easiest way to deal with thigs since
    /// we don't have an easy way to associate information with each vertex.
    /// Asymtotically we're still fine, but real performance will be strictly awful.
    /// The same applies to the spanning tree function.
    /// 
    /// Oh well.
    pub fn shortest_path_len(&mut self, src: Vertex, dst: Vertex) -> usize {
        if !self.sp_cache.contains_key(&src) {
            let mut distances = HashMap::new();
            let mut q = pairing_heap::PairingHeap::new();
            let mut q_handles = HashMap::new();

            // initialize the weights of the non-src vertices to "infinity"
            for v in self.vertices.clone() {
                let weight = if v == src {0} else {usize::max_value()};
                distances.insert(v, weight);
                let handle = q.insert(weight, v);
                q_handles.insert(v, handle);
            }

            while let Some((dist, v)) = q.delete_min() {
                for (neighbor, weight) in (self.adj)(v) {
                    // we onlt want to consider neighbors that are still in the queue
                    if let Some(handle) = q_handles.get(&neighbor) {

                        // can we make a shorter path to neighbor from src through v using this
                        // edge? (and don't overflow)
                        let alt: usize = if dist == usize::max_value() {dist} else {dist + weight};
                        let mut old = distances.get_mut(&neighbor).unwrap();
                        if alt < *old {
                            *old = alt;
                            q.update_key(handle, alt);
                        }
                    }
                }
                q_handles.remove(&v);
            }
            self.sp_cache.insert(src, distances);
        }
        *self.sp_cache.get(&src).unwrap().get(&dst).unwrap()
    }

    /// Return a minimum spanning tree represented as an adjacency list.
    ///
    /// If this function has been called before, the runtime is *O(V + E)*, i.e. the
    /// time it takes to make a copy of the spanning tree, otherwise the runtime is
    /// the that of Prim's algorithm, i.e. *O(E + V log V)*.
    ///
    /// See the pesudocode in the middle of this page, and ignore the fact that the
    /// author's english is barely intelligible (they're probs ESL, so w/e):
    /// http://www.stoimen.com/blog/2012/11/19/computer-algorithms-prims-minimum-spanning-tree/
    pub fn spanning_tree(&'a mut self) -> &'a Vec<Vec<Vertex>> {
        if !self.st_cache.is_some() {
            let mut q = pairing_heap::PairingHeap::new();
            let mut q_handles = HashMap::new();
            let mut parents = HashMap::new();

            // arbitrarily select a root node for our spanning tree, or bail here if the
            // graph is empty
            let root = if let Some(v) = self.vertices.clone().next() {
                v
            } else {
                self.st_cache = Some(Vec::new());
                return self.st_cache.as_ref().unwrap();
            };
            
            // initialize the weights of the non-src vertices to "infinity"
            for v in self.vertices.clone() {
                let weight = if v == root {0} else {usize::max_value()};
                let handle = q.insert(weight, v);
                q_handles.insert(v, handle);
            }

            while let Some((dist, v)) = q.delete_min() {
                for (neighbor, weight) in (self.adj)(v) {
                    // we onlt want to consider neighbors that are still in the queue
                    if let Some(handle) = q_handles.get(&neighbor) {
                        if weight < dist {
                            // set this neighbor's parent to be v
                            parents.remove(&neighbor);
                            parents.insert(neighbor, v);
                            q.update_key(handle, weight);
                        }
                    }
                }
                q_handles.remove(&v);
            }

            // generate the spanning tree from the parent map
            // Is this dumb? Yes.
            // But does it work? Also yes.
            // 
            // Note that with this method we only get directed edges going one way,
            // i.e. child to parent. If we want them the other way, we would need to make
            // a map from vertex values to their indicies, which is ~~annoying~~ and we alreaady
            // have about 11 thousand stupid hash maps in this code and each time I use
            // another one my soul hurts.
            let mut tree = Vec::new();
            for v in self.vertices.clone() {
                let mut adj = Vec::new();
                if let Some(parent) = parents.get(&v) {
                    adj.push(*parent);
                }
                tree.push(adj);
            }

            self.st_cache = Some(tree);
        }
        self.st_cache.as_ref().unwrap()
    }
}
