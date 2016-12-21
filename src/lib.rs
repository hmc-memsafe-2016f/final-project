/// Julien Chien <jchien17@cmc.edu>
///
/// This is a graph data structure that uses adjacency lists

use std::collections::{HashMap,HashSet};
use std::collections::hash_map::Iter;
use std::clone::Clone;


extern crate rand;

// /// Edges have weight w (of whatever data type!)
#[derive(Clone, Debug)]
pub struct Edge<T> {
    start: usize,
    end: usize,
    w: T,
}

// /// Nodes store data of type T
// /// Nodes have an index, which is used in a Graph's adjacency list to refer to nodes
// pub struct Node<T> {
//     data: T,
//     index: usize,
// }

/// Graph has an adjacency list, vector of nodes,
/// and a boolean keeping track of whether it's directed
///
/// adj: a hashmap of node index number to adjacency list to keep track of edges from each node
/// nodes: a hashmap to quickly find nodes given their index number
pub struct Graph<N, E> {
    adj: HashMap<usize, HashMap<usize, Edge<E>>>, //index to its own adjacency list
    nodes: HashMap<usize, N>, //index to Node
    edges: Vec<Edge<E>>,
    directed: bool,
    next_index: usize, //the index of the next new node
}

/// implementation of my graph
impl<N, E:Clone, Eq> Graph<N, E> {

    /// creates a new graph
    pub fn new(d: bool) -> Self {
        Graph{ adj: HashMap::new(), nodes: HashMap::new(), directed: d, next_index: 0 }
    }

    /// returns the number of nodes in the graph
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// returns the number of edges in the graph
    pub fn num_edges(&self) -> usize {
        let mut length = self.adj.iter().fold(0, |acc, (_, m)| acc + m.len());
        if !self.directed {
            length  /= 2; //if undirected, then every edge has two copies
        }
        length
    }

    /// adds a node with data T into the graph
    pub fn add_node(&mut self, data: N) -> usize {
        let index = self.next_index;
        self.nodes.insert(index, data);
        self.adj.insert(index, HashMap::new());
        self.next_index = self.get_next_index();
        index
    }

    /// adds an edge of some weight, going from nodes start to end
    /// if graph is undirected, add_edge will add two edges, one
    /// from start to end, and one from end to start.
    pub fn add_edge(&mut self, start: usize, end: usize, weight: E) {
        assert!(self.nodes.contains_key(&start), "This node does not exist!");
        assert!(self.nodes.contains_key(&end), "This node does not exist!");

        self.adj.get_mut(&start).unwrap().insert(end, Edge{start:start, end:end, w:weight.clone()});
        if !self.directed {
            // if not directed, duplicate another edge going the opposite direction
            self.adj.get_mut(&end).unwrap().insert(start, Edge{start:start, end:end, w:weight.clone()});
        }
        self.edges.push(Edge{start:start, end:end, w:weight.clone()});
    }

    // Returns the data at the node
    pub fn get_node_data(&self, node_index: usize) -> &N {
        assert!(self.nodes.contains_key(&node_index), "This node does not exist!");

        &self.nodes[&node_index]
    }

    pub fn has_edge(&self, start: usize, end: usize) -> bool {
        self.adj[&start].contains_key(&end)
    }

    pub fn get_edge_weight(&self, start: usize, end: usize) -> E {
        assert!(self.has_edge(start, end), "There is no edge from {} to {}", start, end);
        
        self.adj[&start][&end].w.clone()
    }

    pub fn get_nodes<'a>(&'a self) -> Iter<'a, usize, N> {
        self.nodes.iter()
    }

    pub fn get_edges(&self) -> &Vec<Edge<E>> {
        &self.edges
    }

    pub fn get_neighbors<'a>(&'a self, node_index: usize) -> Iter<'a, usize, Edge<E>> {
        self.adj[&node_index].iter()
    }

    pub fn random_node(&self) -> usize {
        let num = rand::random::<(usize)>() % self.nodes.len();
        self.nodes.keys().nth(num).unwrap().clone()
    }

    // gets the next node index number
    fn get_next_index(&self) -> usize {
        let mut curr = 0;
        loop {
            if !self.nodes.contains_key(&curr) {
                break;
            }
            curr += 1;
        }
        curr
    }
}

// Does Prim's algorithm on the graph
// Returns a vetor of a edges that form the minimum spanning tree from Prim's algorithm
pub fn prim<N, E:Clone, Eq>(g: &Graph<N, E>) -> Vec<Edge<E>> {
    let mut included = HashSet::new();
    let mut cost = HashMap::new();
    let mut used_edges = Vec::new();

    let curr = g.random_node();
    included.insert(curr.clone());

    while included.len() != g.num_nodes() {
        let neighbors = g.get_neighbors(curr);
        for (node, edge) in neighbors {
            if !cost.contains_key(node) {
                cost.insert(node, edge);
            } else {
                if edge.w > cost[node].w {
                    cost.insert(node, edge);
                }
            }
        }

        let smallest = E.max();
        let next_edge;
        let next;

        for (node, edge) in cost {
            if 
            if edge.w < smallest {
                smallest = edge.w;
                next = node;
                next_edge = edge;
            }
        }
        curr = next;
        used_edges.push(next_edge.clone());
        included.insert(curr.clone());
    }
    used_edges
}

// Does Kruskal's algorithm on the graph
// Returns a vector of edges that form the minimum spanning tree from Kruskal's algorithm
fn kruskal<N, E:Clone, Eq>(g: &Graph<N, E>) -> Vec<Edge<E>> {
    let mut used_edges = Vec::new();
    let mut included = HashSet::new();
    let mut edges = g.get_edges().clone()
    edges.sort_by(|a, b| a.cmp(b));
    for e in &edges {
        if !included.contains(e.start) || !included.contains(e.end) {
            used_edges.push(e.clone());
            included.insert(e.start.clone());
            included.insert(e.end.clone());
        }
    }
    used_edges
}
