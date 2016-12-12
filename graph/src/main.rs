use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;
use std::collections::HashSet;

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
    edge: &'a Edge
}

/// A NodeIndex identifies a specific node (vertex) in the graph
/// Since nodes cannot be deleted from the graph a NodeIndex will always be valid.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeIndex {
    index: usize // the index in the graph's internal vec's corresponding to the vertex
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
    /// Returns a vector of all the NodeIndex's for all the nodes
    pub fn get_all_nodes(&self) -> Vec<NodeIndex>
    {
        let mut v = vec!();
        for x in 0..self.vertices.len() {
            v.push(NodeIndex{index: x});
        }
        v
    }
}

// State object to help Djikstra's search
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: NodeIndex,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    let mut heap = BinaryHeap::new();
    let mut nodes = g.get_all_nodes();

    // add all nodes to a min heap
    for n in nodes {
        if n == a {
            heap.push(State{cost: 0, node: n});
        } else {
            heap.push(State{cost: usize::max_value(), node: n});
        }
    }

    let mut removed = HashSet::new();
    // remove smallest element and send relaxation offers to neighbors
    while let Some(State {cost, node}) = heap.pop() {
        if removed.contains(&node) {
            continue;
        } else {
            removed.insert(node);
        }
        // if the next smallest element's cost is usize::max_value() then we know
        // that we cannot reach b from a so return usize::max_value()
        if cost == usize::max_value() {
            return usize::max_value();
        }
        if node == b {
            return cost; // we have found a path
        }

        // send "relaxation offers" (since can't decrease key just adding to heap
        // and checking at start whether it's been already added so that we skip those)
        for neighbor in &g.get_neighbors(node) {
            heap.push(State{cost: cost + neighbor.edge.weight, node: neighbor.edge.end});
        }
    }

    usize::max_value() // don't think it should ever reach this code unless b not in graph
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

fn main() {

}