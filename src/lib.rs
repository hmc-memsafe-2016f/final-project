use std::ops::Deref;

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
/// ## Examples
///
/// ```rust
/// use Graph;
/// let g = Graph::<string>::new();
/// let a = g.add_node("a");
/// let b = g.add_node("b");
/// g.add_edge(&a, &b, 5);
/// assert_eq!(g.num_nodes() == 2);
/// assert_eq!(g.num_edges() == 1);
///
/// let result = g.dijkstra(a);
/// assert_eq!(result[0].unwrap(), 0);
/// assert_eq!(result[1].unwrap(), 5);
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
  pub fn add_node(&'a mut self, n: &'a Node) -> NodeIter<'a, Node> {
      for node in &mut self.adjacency_list {
          if node.0 == n {
              panic!("Can't have duplicate nodes.");
          }
      }

      self.adjacency_list.push((n, Vec::new()));
      self.n_nodes += 1;
      NodeIter{node_index: self.adjacency_list.len()-1, graph: self}
  }

  /// Adds a new edge to the graph. Returns an iterator that points to this edge.
  pub fn add_edge(&'a mut self,
              first: &NodeIter<'a, Node>,
              second: &NodeIter<'a, Node>,
              weight: usize)
              { //-> EdgeIter<'a, Node> {
      if first.node_index > self.n_nodes || second.node_index > self.n_nodes {
          panic!("Can't make an edge between one or more nonexistant nodes.");
      }

      match self.graph_type {
          GraphType::Directed => {
              self.adjacency_list[first.node_index].1.push((second.node_index, weight));
          },
          GraphType::Undirected => {
              self.adjacency_list[first.node_index].1.push((second.node_index, weight));
              self.adjacency_list[second.node_index].1.push((first.node_index, weight));
          }
      }

      self.n_edges += 1;
    //   EdgeIter{first: first.node_index, second: second.node_index, graph: self}
  }

  fn create_heap(&'a self, start: NodeIter<'a, Node>) -> (Vec<(Option<usize>, usize)>, Vec<Option<usize>>, Vec<(Option<usize>, NodeIter<'a, Node>)>) {
      let mut heap: Vec<(Option<usize>, usize)> = vec![];
      let mut indices: Vec<Option<usize>> = vec![None; self.n_nodes];
      let mut result: Vec<(Option<usize>, NodeIter<'a, Node>)> = vec![];

      heap.push((Some(0), start.node_index));
      indices[start.node_index] = Some(0);

      for index in 0..self.n_nodes {
          if index < start.node_index {
              heap.push((None, index));
              indices[index] = Some(index + 1);
          } else if index > start.node_index {
              heap.push((None, index));
              indices[index] = Some(index);
          }
          result.push((None, NodeIter{node_index: index, graph: self}));
      }

      (heap, indices, result)
  }

  fn rebalance_heap_up(&self, mut heap_index: usize, heap: &mut Vec<(Option<usize>, usize)>, indices: &mut Vec<Option<usize>>) {
      loop {
          let parent = heap_index / 2;
          let score = heap[heap_index].0.unwrap();
          let heap_node_index = heap[heap_index].1;

          match heap[parent] {
              (Some(parent_score), parent_node_index) if parent_score > score => {
                  heap[heap_index] = (Some(parent_score), parent_node_index);
                  heap[parent] = (Some(score), heap_node_index);
                  indices[heap_node_index] = Some(parent);
                  indices[parent_node_index] = Some(heap_index);
                  heap_index = parent;
              },
              _ => break
          }
      }
  }

  fn rebalance_heap_down(&self, heap: &mut Vec<(Option<usize>, usize)>, indices: &mut Vec<Option<usize>>) {
      let mut heap_index = 0;
      loop {
          let left_child = heap_index * 2;
          let right_child = heap_index * 2 + 1;
          let score = heap[heap_index].0.unwrap();
          let heap_node_index = heap[heap_index].1;

          let mut index = 0;
          let mut node_index = 0;

          if left_child >= heap.len() {
              // no children, do nothing
              break;
          } else if right_child >= heap.len() {
              // Only a left_child
              match heap[left_child] {
                  (Some(child_score), child_index) if child_score < score => {
                      index = left_child;
                      node_index = child_index;
                  },
                  _ => ()
              }
          } else {
              match (heap[left_child], heap[right_child]) {
                  ((None, _), (Some(right_score), right_node_index)) if right_score < score => {
                      index = right_child;
                      node_index = right_node_index;
                  },
                  ((Some(left_score), left_node_index), (None, _)) if left_score < score => {
                      index = left_child;
                      node_index = left_node_index;
                  },
                  ((Some(left_score), left_node_index), (Some(right_score), _))
                    if left_score < right_score && left_score < score => {
                      index = left_child;
                      node_index = left_node_index;
                  },
                  ((Some(left_score), _), (Some(right_score), right_node_index))
                    if left_score > right_score && right_score < score => {
                      index = right_child;
                      node_index = right_node_index;
                  },
                  _ => break
              }
          }

          heap[heap_index] = (heap[index].0, node_index);
          heap[index] = (Some(score), heap_node_index);
          indices[heap_node_index] = Some(index);
          indices[node_index] = Some(heap_index);
          heap_index = index;
      }
  }

  fn relax(&self, score: &usize, heap: &mut Vec<(Option<usize>, usize)>, indices: &mut Vec<Option<usize>>, neighbor: &usize, weight: &usize) {
      match indices[*neighbor] {
          Some(heap_index) => {
              let offer = score + weight;
              match heap[heap_index] {
                  (None, node_index) => {
                      heap[heap_index] = (Some(offer), node_index);
                      self.rebalance_heap_up(heap_index, heap, indices);
                  },
                  (Some(current_best), node_index) if current_best > offer => {
                      heap[heap_index] = (Some(offer), node_index);
                      self.rebalance_heap_up(heap_index, heap, indices);
                  },
                  _ => ()
              }
          },
          _ => ()
      }
  }

  /// Finds the shortest path to all other nodes in a graph
  pub fn dijkstra(&'a self, start: NodeIter<'a, Node>) -> Vec<(Option<usize>, NodeIter<'a, Node>)> {
      let value = self.create_heap(start);
      let mut heap = value.0;
      let mut indices = value.1;
      let mut result = value.2;

      while heap.len() > 0 {
          match heap.swap_remove(0) {
              (None, _) => break,
              (Some(score), index) => {
                  self.rebalance_heap_down(&mut heap, &mut indices);
                  indices[heap[0].1] = Some(0);
                  result[index] = (Some(score), NodeIter{node_index: index, graph: self});
                  indices[index] = None;

                  for n in &self.adjacency_list[index].1 {
                      self.relax(&score, &mut heap, &mut indices, &n.0, &n.1);
                  }
              },
          }
      }
      result
  }

  /// Determines if two graphs are isomorphically equivalent
  pub fn vf2(&self, g2: &Graph<'a, Node>) -> bool {

      unimplemented!()
  }
}

/// Represents the type of a graph, Directed or Undirected
pub enum GraphType {
    Directed,
    Undirected
}
//
// pub struct EdgeIter<'a, N> where N: 'a {
//     first: usize,
//     second: usize,
//     graph: &'a Graph<'a, N>,
// }
//
// impl<'a, N> Deref for EdgeIter<'a, N> {
//     type Target = (NodeIter<'a, N>, NodeIter<'a, N>);
//     fn deref(&self) -> Self::Target {
//         (NodeIter{node_index: self.first, graph: self.graph}, NodeIter{node_index: self.second, graph: self.graph})
//     }
// }

pub struct NodeIter<'a, N> where N: 'a {
    node_index: usize,
    graph: &'a Graph<'a, N>,
}

impl<'a, N> Deref for NodeIter<'a, N> {
    type Target = N;
    fn deref(&self) -> &Self::Target {
        self.graph.adjacency_list[self.node_index].0
    }
}
