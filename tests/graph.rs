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
        use super::super::{Graph, GraphType, NodeIter};
        use std::panic;

        static nodes: [usize;8] = [1, 2, 3, 4, 5, 6, 7, 8];

        #[test]
        fn one_node_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            directed.add_node(&nodes[0]);
            assert_expected_eq_actual!(directed.num_nodes(), 1);
            assert_expected_eq_actual!(directed.num_edges(), 0);
        }

        #[test]
        fn one_node_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            undirected.add_node(&nodes[0]);
            assert_expected_eq_actual!(undirected.num_nodes(), 1);
            assert_expected_eq_actual!(undirected.num_edges(), 0);
        }

        #[test]
        fn multiple_nodes_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            for i in 0..8 {
                directed.add_node(&nodes[i]);
                assert_expected_eq_actual!(directed.num_nodes(), i);
                assert_expected_eq_actual!(directed.num_edges(), 0);
            }
        }

        #[test]
        fn multiple_nodes_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            for i in 0..8 {
                undirected.add_node(&nodes[i]);
                assert_expected_eq_actual!(undirected.num_nodes(), i);
                assert_expected_eq_actual!(undirected.num_edges(), 0);
            }
        }

        #[test]
        fn duplicate_node_panic_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            directed.add_node(&nodes[0]);
            assert_expected_eq_actual!(directed.num_nodes(), 1);
            assert_expected_eq_actual!(directed.num_edges(), 0);

            assert_expected_eq_actual!(panic::catch_unwind(|| {
                directed.add_node(&nodes[0]);
            }).is_err(), true);
        }

        #[test]
        fn duplicate_node_panic_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            undirected.add_node(&nodes[0]);
            assert_expected_eq_actual!(undirected.num_nodes(), 1);
            assert_expected_eq_actual!(undirected.num_edges(), 0);

            assert_expected_eq_actual!(panic::catch_unwind(|| {
                undirected.add_node(&nodes[0]);
            }).is_err(), true);
        }
    }

    mod add_edge {
        use super::super::{Graph, GraphType, NodeIter};

    }
}
