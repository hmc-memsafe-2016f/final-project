extern crate memsafe_graph;

pub use memsafe_graph::{Graph, GraphType, NodeIter};

// This macro is an assertion with nicely formatted failure output
// Borrowed from wk3-starter code
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nExpected `{:?}` is not equal to Actual `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

mod graph {
    static NODES: [usize;8] = [1, 2, 3, 4, 5, 6, 7, 8];

    mod new {
        use super::super::{Graph, GraphType};

        #[test]
        fn empty_directed() {
            let directed = Graph::<()>::new(GraphType::Directed);
            assert_expected_eq_actual!(directed.is_directed(), true);

            assert_expected_eq_actual!(directed.num_nodes(), 0);
            assert_expected_eq_actual!(directed.num_edges(), 0);
        }

        #[test]
        fn empty_undirected() {
            let undirected = Graph::<()>::new(GraphType::Undirected);
            assert_expected_eq_actual!(undirected.is_directed(), false);

            assert_expected_eq_actual!(undirected.num_nodes(), 0);
            assert_expected_eq_actual!(undirected.num_edges(), 0);
        }
    }

    mod add_node {
        use super::super::{Graph, GraphType};
        use super::NODES;

        #[test]
        fn one_node_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            directed.add_node(&NODES[0]);
            assert_expected_eq_actual!(directed.num_nodes(), 1);
            assert_expected_eq_actual!(directed.num_edges(), 0);
        }

        #[test]
        fn one_node_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            undirected.add_node(&NODES[0]);
            assert_expected_eq_actual!(undirected.num_nodes(), 1);
            assert_expected_eq_actual!(undirected.num_edges(), 0);
        }

        #[test]
        fn multiple_nodes_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            for i in 0..8 {
                directed.add_node(&NODES[i]);
                assert_expected_eq_actual!(directed.num_nodes(), i+1);
                assert_expected_eq_actual!(directed.num_edges(), 0);
            }
        }

        #[test]
        fn multiple_nodes_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            for i in 0..8 {
                undirected.add_node(&NODES[i]);
                assert_expected_eq_actual!(undirected.num_nodes(), i+1);
                assert_expected_eq_actual!(undirected.num_edges(), 0);
            }
        }

        #[test]
        #[should_panic]
        fn duplicate_node_panic_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            directed.add_node(&NODES[0]);
            directed.add_node(&NODES[0]);
        }

        #[test]
        #[should_panic]
        fn duplicate_node_panic_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            undirected.add_node(&NODES[0]);
            undirected.add_node(&NODES[0]);
        }
    }

    mod add_edge {
        use super::super::{Graph, GraphType, NodeIter};
        use super::NODES;

        #[test]
        fn one_edge_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let a = directed.add_node(&NODES[0]);
            let b = directed.add_node(&NODES[1]);
            directed.add_edge(&a, &b, 10);
            assert_expected_eq_actual!(directed.num_edges(), 1);
        }

        #[test]
        fn one_edge_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let a = undirected.add_node(&NODES[0]);
            let b = undirected.add_node(&NODES[1]);
            undirected.add_edge(&a, &b, 10);
            assert_expected_eq_actual!(undirected.num_edges(), 1);
        }

        #[test]
        fn multiple_edge_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let mut iters = vec![];
            for i in 0..8 {
                iters.push(directed.add_node(&NODES[i]));
                assert_expected_eq_actual!(directed.num_nodes(), i+1);
                assert_expected_eq_actual!(directed.num_edges(), 0);
            }

            for i in 0..4 {
                directed.add_edge(&iters[(2*i)], &iters[(2*i) + 1], 10);
                assert_expected_eq_actual!(directed.num_edges(), i+1);
            }
        }

        #[test]
        fn multiple_edge_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let mut iters = vec![];
            for i in 0..8 {
                iters.push(undirected.add_node(&NODES[i]));
                assert_expected_eq_actual!(undirected.num_nodes(), i+1);
                assert_expected_eq_actual!(undirected.num_edges(), 0);
            }

            for i in 0..4 {
                undirected.add_edge(&iters[(2*i)], &iters[(2*i) + 1], 10);
                assert_expected_eq_actual!(undirected.num_edges(), i+1);
            }
        }

        #[test]
        #[should_panic]
        fn duplicate_edge_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let a = directed.add_node(&NODES[0]);
            let b = directed.add_node(&NODES[1]);
            directed.add_edge(&a, &b, 10);
            directed.add_edge(&a, &b, 10);
        }

        #[test]
        fn edge_both_ways_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let a = directed.add_node(&NODES[0]);
            let b = directed.add_node(&NODES[1]);
            directed.add_edge(&a, &b, 10);
            directed.add_edge(&b, &a, 10);
            assert_expected_eq_actual!(directed.num_edges(), 2);
        }

        #[test]
        #[should_panic]
        fn duplicate_edge_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let a = undirected.add_node(&NODES[0]);
            let b = undirected.add_node(&NODES[1]);
            undirected.add_edge(&a, &b, 10);
            undirected.add_edge(&a, &b, 10);
        }

        #[test]
        #[should_panic]
        fn edge_both_ways_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let a = undirected.add_node(&NODES[0]);
            let b = undirected.add_node(&NODES[1]);
            undirected.add_edge(&a, &b, 10);
            undirected.add_edge(&b, &a, 10);
        }

        #[test]
        #[should_panic]
        fn invalid_node_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let a = directed.add_node(&NODES[0]);
            let b = NodeIter::new(1000);
            directed.add_edge(&a, &b, 10);
        }

        #[test]
        #[should_panic]
        fn invalid_node_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let a = undirected.add_node(&NODES[0]);
            let b = NodeIter::new(1000);
            undirected.add_edge(&a, &b, 10);
        }
    }
}
