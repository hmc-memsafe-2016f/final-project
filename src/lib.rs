use std::usize;
use std::collections::BinaryHeap;

/// Graph data type for MemSafety.
///
/// Provides a clean and simple graph API for a graph
/// with arbitrary node and edge types, as well as
/// either directed or undirected graphs.
///
/// Multigraphs are not allowed, and will cause errors.
///
/// All operations involving pre-existing nodes or edges
/// will panic if the assumed node or edge does not exist.
///
/// ```
pub struct Graph<'a, Node> where Node: 'a {
    // This will be some data members, for example
    adjacency_list: Vec<(&'a Node, Vec<(usize, usize)>)>,
    n_nodes: usize,
    n_edges: usize,
    graph_type: GraphType,
}

impl<'a, Node: Eq> Graph<'a, Node> {
  /// Constructs a new graph of a given direction type
  pub fn new(g: GraphType) -> Self {
      Graph{adjacency_list: vec![], n_nodes: 0, n_edges: 0, graph_type: g}
  }

  /// Returns whether or not the graph is directed.
  pub fn is_directed(&self) -> bool {
      match self.graph_type {
          GraphType::Directed => true,
          _ => false
      }
  }

  /// Return the number of nodes in the graph in O(1) time.
  pub fn num_nodes(&self) -> usize {
      self.n_nodes
  }

  /// Return the number of edges in the graph in O(1) time.
  pub fn num_edges(&self) -> usize {
      self.n_edges
  }

  /// Adds a new node to the graph. Returns an iterator that points to this node.
  pub fn add_node<'b>(&'b mut self, n: &'a Node) -> NodeIter where 'a: 'b {
      for node in &mut self.adjacency_list {
          if node.0 == n {
              panic!("Can't have duplicate nodes.");
          }
      }

      self.adjacency_list.push((n, Vec::new()));
      self.n_nodes += 1;
      NodeIter::new(self.adjacency_list.len()-1)
  }

  /// Adds a new edge to the graph. Returns an iterator that points to this edge.
  pub fn add_edge<'b, 'c: 'b>(&'b mut self,
              first: &'c NodeIter,
              second: &'c NodeIter,
              weight: usize) where 'a: 'b, 'a: 'c
              { //-> EdgeIter<'a, Node> {
      if first.node_index > self.n_nodes || second.node_index > self.n_nodes {
          panic!("Can't make an edge between one or more nonexistant nodes.");
      }

      match self.graph_type {
          GraphType::Directed => {
              for node in &self.adjacency_list[first.node_index].1 {
                  match node {
                      &(node_index, _) => assert!(node_index != second.node_index)
                  }
              }
              self.adjacency_list[first.node_index].1.push((second.node_index, weight));
          },
          GraphType::Undirected => {
              for node in &self.adjacency_list[first.node_index].1 {
                  match node {
                      &(node_index, _) => assert!(node_index != second.node_index)
                  }
              }
              self.adjacency_list[first.node_index].1.push((second.node_index, weight));
              for node in &self.adjacency_list[second.node_index].1 {
                  match node {
                      &(node_index, _) => assert!(node_index != first.node_index)
                  }
              }
              self.adjacency_list[second.node_index].1.push((first.node_index, weight));
          }
      }

      self.n_edges += 1;
    //   EdgeIter{first: first.node_index, second: second.node_index, graph: self}
  }

  /// Find the shortest path from some node to every other node in the graph
  pub fn dijkstra(&mut self, start: &NodeIter) -> Vec<(usize, NodeIter)> {
    let mut results: Vec<(usize, NodeIter)> = (0..self.adjacency_list.len()).map(|i| (usize::MAX, NodeIter::new(i))).collect();

    let mut heap = BinaryHeap::new();

    results[start.node_index].0 = 0;
    heap.push((0, start.node_index));

    while let Some((cost, node_index)) = heap.pop() {
        // If we can't improve, then move along
        if cost > results[node_index].0 {
            continue;
        }

        // Otherwise, make a relaxation offer to all neighbors
        for &(neighbor, weight) in &self.adjacency_list[node_index].1 {
            if cost + weight < results[neighbor].0 {
                heap.push((cost + weight, neighbor));
                results[neighbor] = (cost + weight, NodeIter::new(neighbor));
            }
        }
    }

    results
  }


  /// Determines if two graphs are isomorphically equivalent
  /// 
  /// This ended up being a rather hybridized version of the VF2 algorithm;
  /// it brute forces everything, while I made a few adjustments to avoid
  /// some of the extra work
  pub fn vf2(&self, g2: &Graph<'a, Node>) -> bool {
      // If they aren't the same size, obviously not isomorphic
      if self.num_nodes() != g2.num_nodes() || self.num_edges() != g2.num_edges() {
          return false;
      }

      // Both must be directed (orundirected)
      if self.is_directed() != g2.is_directed() {
          return false;
      }

      // empty graphs are obviously isomorphic
      if self.num_nodes() == 0 {
          return true;
      }

      // edgeless graphs are obviously isomorphic
      if self.num_edges() == 0 {
          return true;
      }

      let mut isomorphism: Vec<Option<usize>> = (0..self.num_nodes()).map(|_| None).collect();
      let mut rev_isomorphism: Vec<Option<usize>> = (0..self.num_nodes()).map(|_| None).collect();

      // This makes sure we get all connected components
      loop {
          match isomorphism.iter()
                           .enumerate()
                           .filter(|n| n.1.is_none())
                           .map(|n| n.0)
                           .next() {
              Some(start) => {
                  let mut options = vec![];
                  for index in 0..g2.adjacency_list.len() {
                      let &(_, ref list) = &g2.adjacency_list[index];
                      if list.len() == self.adjacency_list[start].1.len() && rev_isomorphism[index].is_none() {
                          options.push(index);
                      }
                  }

                  if !self.vf2_impl(g2, start, &options, &mut isomorphism, &mut rev_isomorphism) {
                      // If we get here, that means that we weren't able to complete the isomorphism
                      // for this connected component, which means we can quit now
                      return false;
                  }
              },
              None => break,
          }
      }

      isomorphism.iter().all(|n| n.is_some()) && rev_isomorphism.iter().all(|n| n.is_some())

  }

  fn vf2_impl(&self, g2: &Graph<'a, Node>, current: usize, possibilities: &Vec<usize>, isomorphism: &mut Vec<Option<usize>>, reverse_isomorphism: &mut Vec<Option<usize>>) -> bool {
      println!("A: B | C: D, current={}", current);
      for index in 0..self.num_nodes() {
          println!("{}: {:?} | {}: {:?}", index, isomorphism[index], index, reverse_isomorphism[index]);
      }
      println!("Possible neighbors");
      for neighbor in possibilities {
          println!("{}", neighbor);
      }
      // try each of the possibilities as an isomorph
      for &possible_isomorph in possibilities {
          // Verify that it isn't already being used in the isomorphism
          assert!(isomorphism[current].is_none());
          assert!(reverse_isomorphism[possible_isomorph].is_none());

          isomorphism[current] = Some(possible_isomorph);
          reverse_isomorphism[possible_isomorph] = Some(current);
          let my_num_neighbors = self.adjacency_list[current].1
                                 .iter()
                                 .filter(|n| isomorphism[n.0].is_none())
                                 .count();
          let your_num_neighbors = g2.adjacency_list[possible_isomorph].1
                                 .iter()
                                 .filter(|n| reverse_isomorphism[n.0].is_none())
                                 .count();

          // if we've both exhausted our options, then its all agood
          if my_num_neighbors == 0 && your_num_neighbors == 0 {
              return true;
          } else if my_num_neighbors == 0 {
              isomorphism[current] = None;
              reverse_isomorphism[possible_isomorph] = None;
              continue;
          }

          // This is tricky, but borrowck gets mad because vf2_impl takes a mutable
          // reference to isomorphism
          // However, our recursive calls might mutate which of our neighbors are available
          // to be used (i.e. in a cycle).
          // Thus we simply loop infinitely, and each time we grab the first available neighbor.
          let my_neighbors: Vec<&(usize, usize)> = self.adjacency_list[current].1
                                                      .iter()
                                                      .filter(|n| isomorphism[n.0].is_none())
                                                      .collect();
          let mut index = 0;
          'neighbors: loop {
              while isomorphism[my_neighbors[index].0].is_some() {
                  index += 1;
                  if index >= my_neighbors.len() {
                      break 'neighbors;
                  }
              }

              let &(ref my_neighbor, ref my_neighbor_weight) = my_neighbors[index];

              let options: Vec<usize> = g2.adjacency_list[possible_isomorph].1
                                          .iter()
                                          .filter(|n| {
                                              // Not already used by the isomorphism
                                              (reverse_isomorphism[n.0].is_none() 
                                               // The edge between them matches in terms of weight
                                               && n.1 == *my_neighbor_weight 
                                               // And they have the same number of neighbors
                                               && g2.adjacency_list[n.0].1.len() == self.adjacency_list[*my_neighbor].1.len())
                                          })
                                          .map(|n| n.0)
                                          .collect();
          
              // If we couldn't find a valid isomorph here, that means this iteration is incorrect
              // and we can say that the given possible_isomorph is accurate
              // If we did, then isomorphism and reverse_isomorphism will both have the appropriate
              // values in them
              if self.vf2_impl(g2, *my_neighbor, &options, isomorphism, reverse_isomorphism) {
                  return true;
              }
              index += 1;
          }

          isomorphism[current] = None;
          reverse_isomorphism[possible_isomorph] = None;
      }
      
      false
  }
}

/// Represents the type of a graph, Directed or Undirected
pub enum GraphType {
    Directed,
    Undirected
}

/// An iterator-like object that contains some reference to a given node in the graph.
pub struct NodeIter {
    node_index: usize,
}

impl NodeIter {
    /// Create a new node iterator
    pub fn new(index: usize) -> Self {
        NodeIter{node_index: index}
    }

    /// Access the actual index. For testing purposes only, not intended to be used
    /// by users
    pub fn get_index(&self) -> usize {
        self.node_index
    }
}
