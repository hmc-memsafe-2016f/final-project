use std::collections::HashMap;
use std::collections::hash_map;

/// Graph that stores data for each edge and vertex
///
/// ## Examples
///
/// ```
/// use graph::Graph;
/// struct V{ data: usize }
/// struct E{ data: usize }
/// let mut g = Graph::new();
///
/// let v1 = g.add_vertex(V{data: 4});
/// let v2 = g.add_vertex(V{data: 6});
/// let v3 = g.add_vertex(V{data: 7});
/// g.add_edge(v2, v1, E{data: 8});
/// ```
#[derive(Debug, Clone)]
pub struct Graph<V, E> {
    vertices: Vec<V>,
    edge_data: Vec<E>,
    edges: Vec<HashMap<usize, usize>>,
}

impl<V, E> Graph<V, E>
where E: std::fmt::Debug {
    /// Creates a new empty graph.
    pub fn new() -> Graph<V, E> {
        Graph{vertices: Vec::new(), edge_data: Vec::new(), edges: Vec::new()}
    }
    /// Adds a vertex to the graph.
    pub fn add_vertex(&mut self, data: V) -> usize {
        self.vertices.push(data);
        self.edges.push(HashMap::new());
        self.vertices.len() - 1
    }
    /// Adds an edge between two vertices. The vertices must be added first.
    pub fn add_edge(&mut self, u: usize, v: usize, data: E) {
        assert!(u < self.vertices.len());
        assert!(v < self.vertices.len());

        self.edges[u].insert(v, self.edge_data.len());
        self.edges[v].insert(u, self.edge_data.len());
        self.edge_data.push(data);
    }
    /// Returns a reference to the edge betwen u and v if one exists, otherwise
    /// None. This can be used to check if vertices are adjacent.
    pub fn get_edge(&self, u: usize, v: usize) -> Option<&E> {
        assert!(u < self.vertices.len());
        assert!(v < self.vertices.len());

        self.edges[u].get(&v).map(|&n|&self.edge_data[n])
    }
    /// Returns the given vertex.
    pub fn get_vertex(&self, v: usize) -> &V {
        &self.vertices[v]
    }
    /// Returns all the vertices.
    pub fn vertices(&self) -> &Vec<V> {
        &self.vertices
    }
    /// Returns an iterator to the vertices that are neighbors of v.
    pub fn neighbors<'a>(&'a self, v: usize) -> hash_map::Keys<'a, usize, usize> {
        self.edges[v].keys()
    }
    /// Returns an iterator to the edges that are incident to v.
    pub fn incident_edges<'a>(&'a self, v: usize) -> IncidentEdges<'a, E> {
        IncidentEdges{edge_data: &self.edge_data, it: self.edges[v].values()}
    }
    /// Returns an iterator to the vertices that are the neihbors. The return
    /// value is a tuple of the vertex number and the edge
    pub fn neighbors_with_edge<'a>(&'a self, v: usize) -> NeighborsWithEdges<'a, E> {
        NeighborsWithEdges{edge_data: &self.edge_data, it: self.edges[v].iter()}
    }
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
    pub fn num_edges(&self) -> usize {
        self.edges.iter().fold(0, |x, m| m.len() + x) / 2
    }
}

#[derive(Clone)]
pub struct NeighborsWithEdges<'a, E>
        where E: 'a
{
    edge_data: &'a Vec<E>,
    it: hash_map::Iter<'a, usize, usize>
}

impl<'a, E> Iterator for NeighborsWithEdges<'a, E>
{

    type Item = (usize, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|(&n, &e)| (n.clone(), &self.edge_data[e]))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
impl<'a, E> ExactSizeIterator for NeighborsWithEdges<'a, E> {}

#[derive(Clone)]
pub struct IncidentEdges<'a, E>
        where E: 'a
{
    edge_data: &'a Vec<E>,
    it: hash_map::Values<'a, usize, usize>
}

impl<'a, E> Iterator for IncidentEdges<'a, E>
{

    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|&n| &self.edge_data[n])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
impl<'a, E> ExactSizeIterator for IncidentEdges<'a, E> {}
