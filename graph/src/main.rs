/// This is an adjacency list implemented directed weighted graph
pub struct Graph<T> {
    adjacency_list: Vec<Vec<Edge>>,
    vertices: Vec<T>
}

/// This Edge structure represents an edge in a graph
pub struct Edge {
    weight: usize,
    start: NodeIndex,
    end: NodeIndex
}

/// An EdgeIndex identifies a specific edge in the graph
pub struct EdgeIndex<'a> {
    edge: &'a Edge // TODO: figure out why do I need this lifetime parameter
}

/// A NodeIndex identifies a specific node (vertex) in the graph
pub struct NodeIndex {
    index: usize // the index in the graph's internal vec's corresponding to the vertex
}

/// Uses djikstra's algorithm to find the min path from a to b. Returns length of path
/// or the max value of a usize if there is no path.
pub fn djikstra<T>(g : &Graph<T>, a: NodeIndex, b: NodeIndex) -> usize
{
    /*
    Djikstra's algorithm works is like this:
    Add all vertices to a priority Q and give them a distance of infinity
    except for a which will be given a distance of 0 (since it's the start).
    Remove the smallest element from the priority Q and send relaxation offers
    to its neighbors (allow them to take on the value of current elem's dist
    plus edge weight to get to them if they want).
    Repeat until we reach vertex b at which point we return b's distance.
    */
    unimplemented!()
}

/// Does a topological sort of the graph.
/// Will return the order that the nodes can be visited unless it is not possible
/// to topologically sort the graph in which case it will return None
pub fn topo_sort<T>(g : &Graph<T>) -> Option<&[NodeIndex]>
{
    /*
    To implement this I will first go through the graph and count the in degree
    of each vertex (that is how many edges are going in to them).
    Then I will find a vertex with in degree 0 (if we cannot return None).
    Then I will add that vertex to the slice I will return, decrement the
    in degrees of each node it has an edge to, and repeat until we havee added
    all vertices to the slice we are returning.
    */
    unimplemented!()
}

impl<T> Graph<T> {
    /// Creates a new `Graph`
    pub fn new() -> Self
    {
        Graph{adjacency_list : vec!(), vertices: vec!()}
    }
    /// Adds a node to the graph and returns its NodeIndex
    pub fn add_node(&mut self, data: T) -> NodeIndex
    {
        self.vertices.push(data);
        self.adjacency_list.push(vec!());

        // Sanity check
        assert!(self.vertices.len() == self.adjacency_list.len()
            , "vertices and adjacency list should be same size");
        assert!(self.vertices.len() > 0);

        NodeIndex{index: self.vertices.len() - 1}
    }
    /// Adds a directed edge from a to b with weight w
    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, w: usize) -> EdgeIndex
    {
        let adjL_index = a.index;
        self.adjacency_list[adjL_index].push(Edge{weight: w, start: a, end: b});
        EdgeIndex{edge: self.adjacency_list[adjL_index].last().unwrap()}
    }
    /// Returns all edges going out of a given node
    pub fn get_neighbors(&self, node: NodeIndex) -> Vec<EdgeIndex>
    {
        let mut v = vec!();
        for n in &self.adjacency_list[node.index] {
            v.push(EdgeIndex{edge: &n});
        }
        v
    }
}


fn main() {

}