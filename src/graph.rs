use std::vec::IntoIter;
use std::ops::Range;

/// A simple datastructure for modelling directed graphs, backed by an
/// adjacency matrix.
///
/// Vertices and edges can hold arbitrary associated data (called "weights", in
/// the case of edges).
///
/// The API refers to vertices using `Vertex` indices. Similarly, edges are
/// referred to simply by pairs of indices. **All such `Vertex` indices are
/// invalidated by the `remove_vertex` method.**
///
/// ## Example Usage
///
/// ```
/// use graph_api::Graph;
/// let mut g = Graph::new();
/// let a = g.add_vertex("a");
/// let b = g.add_vertex("b");
/// g.add_edge(a, b, "a -> b");
/// assert!(g.has_edge(a,b));
/// assert!(!g.has_edge(b,a));
/// assert_eq!(g.vertex_data(b), &"b");
/// assert_eq!(g.weight(a,b), &"a -> b");
/// ```
pub struct Graph<V, E> {
    matrix: Vec<Vec<Option<E>>>,
    vertices: Vec<V>,
    size: usize,
}

/// Indices into vertices of a `Graph`.
///
/// Careful: `Vertex` indices are invalidated by the `remove_vertex` method.
pub type Vertex = usize;

/// An edge *(u,v)* from vertex *u* to *v*.
///
/// Careful: `Vertex` indices are invalidated by the `remove_vertex` method.
pub type Edge = (Vertex, Vertex);

/// An edge *(u,v,w)* from vertex *u* to *v*, with weight *w*.
///
/// Careful: `Vertex` indices are invalidated by the `remove_vertex` method.
pub type WeightedEdge<'a, E> = (Vertex, Vertex, &'a E);

impl<V, E> Graph<V, E> {
    /// Creates a new graph with no vertices.
    pub fn new() -> Self {
        Graph {
            matrix: Vec::new(),
            vertices: Vec::new(),
            size: 0,
        }
    }

    /// Returns the number of vertices in the graph.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds a vertex with associated data `data` to the graph. Returns the
    /// index of the new vertex.
    ///
    /// Does not invalidate existing `Vertex` indices.
    pub fn add_vertex(&mut self, data: V) -> Vertex {
        self.size += 1;
        for row in &mut self.matrix {
            row.push(None)
        }
        let mut row = Vec::new();
        for _ in 0..self.size {
            row.push(None)
        }
        self.vertices.push(data);
        self.matrix.push(row);
        self.size - 1
    }

    /// Adds an edge with associated data `weight` to the graph.
    ///
    /// Overwrites any previous edge from `from` to `to`.
    ///
    /// **Panics** if `from` or `to` does not exist in the graph.
    pub fn add_edge(&mut self, from: Vertex, to: Vertex, weight: E) {
        self.matrix[from][to] = Some(weight);
    }

    /// Whether the edge exists between the two vertices.
    ///
    /// Note that this function will not report a reflexive edge from a vertex
    /// to itself unless such an edge has been explicitly added with
    /// `add_vertex`.
    pub fn has_edge(&self, from: Vertex, to: Vertex) -> bool {
        self.matrix[from][to].is_some()
    }

    /// Returns a reference to the data associated with a vertex.
    ///
    /// **Panics** if `index` does not exist in the graph.
    pub fn vertex_data(&self, index: Vertex) -> &V {
        &self.vertices[index]
    }

    /// Returns a reference to the weight associated with an edge.
    ///
    /// **Panics** if the edge does not exist in the graph.
    pub fn weight(&self, from: Vertex, to: Vertex) -> &E {
        &self.matrix[from][to].as_ref().unwrap()
    }

    /// Returns an iterator of all vertices in the graph.
    pub fn vertices(&self) -> Vertices {
        Vertices::new(self)
    }

    /// Returns an iterator of all edges in the graph.
    pub fn edges(&self) -> Edges {
        Edges::new(self)
    }

    /// Removes a vertex from the graph and returns its associated data.
    ///
    /// **Panics** if `index` does not exist in the graph.
    pub fn remove_vertex(&mut self, index: Vertex) -> V {
        self.matrix.remove(index);
        for row in &mut self.matrix {
            row.remove(index);
        }
        self.size -= 1;
        self.vertices.remove(index)
    }

    /// Removes an edge from the graph and returns its associated weight data.
    ///
    /// **Panics** if the edge does not exist in the graph.
    pub fn remove_edge(&mut self, from: Vertex, to: Vertex) -> E {
        self.matrix[from][to].take().unwrap()
    }
}

impl<V, E> Graph<V, E>
    where E: Copy
{
    /// Convenience method to add a bidirectional edge (equivalent to adding
    /// both the edges `(from, to)` and `(to, from)`).
    pub fn add_bidi_edge(&mut self, from: Vertex, to: Vertex, weight: E) {
        self.add_edge(from, to, weight);
        self.add_edge(to, from, weight);
    }
}

/// Iterator over the vertices of a graph.
pub struct Vertices {
    iter: Range<usize>,
}

impl Vertices {
    fn new<V, E>(graph: &Graph<V, E>) -> Vertices {
        Vertices { iter: 0..graph.size() }
    }
}

/// Iterator over the edges of a graph.
pub struct Edges {
    iter: IntoIter<Edge>,
}

impl Iterator for Vertices {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Edges {
    fn new<V, E>(graph: &Graph<V, E>) -> Edges {
        let mut edges = Vec::new();
        let mut from = 0;
        let mut to = 0;
        while from < graph.size() {
            if let Some(_) = graph.matrix[from][to] {
                edges.push((from, to));
            }
            to += 1;
            if to >= graph.size {
                from += 1;
                to = 0;
            }
        }
        Edges { iter: edges.into_iter() }
    }
}

impl Iterator for Edges {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
