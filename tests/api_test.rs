extern crate mjgraph;

pub use mjgraph::{Graph};

macro_rules! assert_expected_eq_actual {
  ($a:expr, $b:expr) => ({
    let (a, b) = (&$a, &$b);
      assert!(*a == *b,
        "\nExpected `{:?}` is not equal to Actual `{:?}`\n\
        Assertion: `assert_expected_eq_actual!({}, {})`",
        *a,
        *b,
        stringify!($a),
        stringify!($b));
       })
}

mod api_test {
  mod construct_graph {
    use super::super::{Graph};
    #[test]
    fn simple() {
      let g : Graph<u8,u8> = Graph::new();
      let expected_adj_mat : Vec<usize> = Vec::new();
      assert_expected_eq_actual!(0, g.num_vertices());
      assert_expected_eq_actual!(0, g.num_edges());
      assert_expected_eq_actual!(expected_adj_mat, g.get_adjacency_matrix());
    }
    #[test]
    fn construct_from_list_and_with_add_edge() {
      //  Picture of the graph, - is edge, = is double edge, c is loop
      //  (0) = (1)
      //   |     |
      // c(3) - (2)
      let edges = vec![(0,1),(1,0),(1,2),(2,3),(0,3),(3,3)];
      let expected_adj_mat = vec![0, 1, 0, 1,  // not symmetric because
                                  1, 0, 1, 0,  // we have a digraph
                                  0, 0, 0, 1,
                                  0, 0, 0, 1];
      let g_from_list : Graph<usize,()> = Graph::new();
      g_from_list.extend_with_edges(&edges);
      let g : Graph<usize,()> = Graph::new();
      let mut vertex_iters = Vec::new();
      for v in 0..3 {
        vertex_iters.push(g.add_vertex(v));
      }
      for (u, v) in edges {
        g.add_directed_edge(&vertex_iters[u], &vertex_iters[v], ());
      }
      assert_expected_eq_actual!(expected_adj_mat, g_from_list.get_adjacency_matrix());
      assert_expected_eq_actual!(expected_adj_mat, g.get_adjacency_matrix());
    }
  }
  mod get_values {
    use super::super::{Graph};
    #[test]
    fn get_vertices() {
      let edges = vec![(2,3),(3,2),(3,4),(4,5),(2,5),(5,5)];
      let g: Graph<usize,&str> = Graph::new();
      let refs = g.extend_with_edges(&edges);
      for i in 0..3 {
        // The value of each vertex should be the vertex in the order
        // encountered: 2,3,4,5
        assert_expected_eq_actual!(i + 2, *refs[i]);
      let new_edge = g.add_directed_edge(&refs[0], &refs[2], "new_edge");
      let edge_name = (*new_edge).weight;
      assert_expected_eq_actual!("new_edge", edge_name);
      }
    }
  }
}
