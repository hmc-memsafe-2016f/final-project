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

mod algorithms {
    static NODES: [u8; 6] = [1, 2, 3, 4, 5, 6];
    mod vf2 {
        use super::super::{Graph, GraphType};
        use super::NODES;

        #[test]
        fn directed_empty() {
            let first = Graph::<()>::new(GraphType::Directed);
            let second = Graph::new(GraphType::Directed);

            assert_expected_eq_actual!(first.vf2(&second), true);

        }

        #[test]
        fn directed_diff_num_edges() {
            let mut first = Graph::new(GraphType::Directed);
            let mut second = Graph::new(GraphType::Directed);

            let mut fiters = vec![];
            fiters.push(first.add_node(&NODES[0]));
            fiters.push(first.add_node(&NODES[1]));
            first.add_edge(&fiters[0], &fiters[1], 10);

            let mut siters = vec![];
            siters.push(second.add_node(&NODES[4]));
            siters.push(second.add_node(&NODES[5]));

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn directed_diff_num_nodes() {
            let mut first = Graph::new(GraphType::Directed);
            let mut second = Graph::new(GraphType::Directed);

            let mut fiters = vec![];
            fiters.push(first.add_node(&NODES[0]));
            fiters.push(first.add_node(&NODES[1]));

            let mut siters = vec![];
            siters.push(second.add_node(&NODES[4]));

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn undirected_empty() {
            let first = Graph::<()>::new(GraphType::Undirected);
            let second = Graph::new(GraphType::Undirected);

            assert_expected_eq_actual!(first.vf2(&second), true);
        }

        #[test]
        fn undirected_diff_num_edges() {
            let mut first = Graph::new(GraphType::Undirected);
            let mut second = Graph::new(GraphType::Undirected);

            let mut fiters = vec![];
            fiters.push(first.add_node(&NODES[0]));
            fiters.push(first.add_node(&NODES[1]));
            first.add_edge(&fiters[0], &fiters[1], 10);

            let mut siters = vec![];
            siters.push(second.add_node(&NODES[4]));
            siters.push(second.add_node(&NODES[5]));

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn undirected_diff_num_nodes() {
            let mut first = Graph::new(GraphType::Undirected);
            let mut second = Graph::new(GraphType::Undirected);

            let mut fiters = vec![];
            fiters.push(first.add_node(&NODES[0]));
            fiters.push(first.add_node(&NODES[1]));

            let mut siters = vec![];
            siters.push(second.add_node(&NODES[4]));

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn diff_directedness() {
            let first = Graph::<()>::new(GraphType::Undirected);
            let second = Graph::<()>::new(GraphType::Directed);

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn isomorphic_directed() {
            let mut first = Graph::new(GraphType::Directed);
            let mut fiters = vec![];
            for i in 0..6 {
                fiters.push(first.add_node(&NODES[i]))
            }

            for i in 0..3 {
                first.add_edge(&fiters[(2*i)], &fiters[(2*i) + 1], i);
            }

            // This graph looks like
            // 1->2
            // 3->4
            // 5->6

            let mut second = Graph::new(GraphType::Directed);
            let mut siters = vec![];

            for i in 0..6 {
                siters.push(second.add_node(&NODES[6-i-1]));
            }

            for i in 0..3 {
                second.add_edge(&siters[(2*i)], &siters[(2*i)+1], i);
            }

            // This graph looks like
            // 6->5
            // 4->3
            // 2->1

            // One of the isomorphism would be
            // 1->6
            // 2->5
            // 3->4
            // 4->3
            // 5->2
            // 1->6
            assert_expected_eq_actual!(first.vf2(&second), true);
        }

        #[test]
        fn isomorphic_undirected() {
            let mut first = Graph::new(GraphType::Undirected);
            let mut fiters = vec![];
            for i in 0..6 {
                fiters.push(first.add_node(&NODES[i]))
            }

            for i in 0..3 {
                first.add_edge(&fiters[(2*i)], &fiters[(2*i) + 1], i);
            }

            // This graph looks like
            // 1->2
            // 3->4
            // 5->6

            let mut second = Graph::new(GraphType::Undirected);
            let mut siters = vec![];

            for i in 0..6 {
                siters.push(second.add_node(&NODES[6-i-1]));
            }

            for i in 0..3 {
                second.add_edge(&siters[(2*i)], &siters[(2*i)+1], i);
            }

            // This graph looks like
            // 6->5
            // 4->3
            // 2->1

            // One of the isomorphism would be
            // 1->6
            // 2->5
            // 3->4
            // 4->3
            // 5->2
            // 1->6
            assert_expected_eq_actual!(first.vf2(&second), true);
        }

        #[test]
        fn not_isomorphic_directed() {
            let mut first = Graph::new(GraphType::Directed);
            let mut fiters = vec![];
            for i in 0..6 {
                fiters.push(first.add_node(&NODES[i]))
            }

            for i in 0..3 {
                first.add_edge(&fiters[(2*i)], &fiters[(2*i) + 1], i);
            }
            // This graph looks like
            // 1->2
            // 3->4
            // 5->6

            let mut second = Graph::new(GraphType::Directed);
            let mut siters = vec![];

            for i in 0..6 {
                siters.push(second.add_node(&NODES[6-i-1]));
            }

            for i in 0..3 {
                second.add_edge(&siters[0], &siters[(2*i)+1], i);
            }
            // This graph looks like
            // 6->5
            // 6->3
            // 6->1
            // 4
            // 2

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

        #[test]
        fn not_isomorphic_undirected() {
            let mut first = Graph::new(GraphType::Undirected);
            let mut fiters = vec![];
            for i in 0..6 {
                fiters.push(first.add_node(&NODES[i]))
            }

            for i in 0..3 {
                first.add_edge(&fiters[(2*i)], &fiters[(2*i) + 1], i);
            }
            // This graph looks like
            // 1->2
            // 3->4
            // 5->6

            let mut second = Graph::new(GraphType::Undirected);
            let mut siters = vec![];

            for i in 0..6 {
                siters.push(second.add_node(&NODES[6-i-1]));
            }

            for i in 0..3 {
                second.add_edge(&siters[0], &siters[(2*i)+1], i);
            }
            // This graph looks like
            // 6->5
            // 6->3
            // 6->1
            // 4
            // 2

            assert_expected_eq_actual!(first.vf2(&second), false);
        }

    }

    mod dijkstra {
        use super::super::{Graph, GraphType};
        use super::NODES;
        use std::usize;

        #[test]
        fn connected_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let mut iters = vec![];
            for i in 0..6 {
                iters.push(directed.add_node(&NODES[i]));
            }

            directed.add_edge(&iters[0], &iters[1], 2);
            directed.add_edge(&iters[0], &iters[3], 4);
            directed.add_edge(&iters[1], &iters[2], 8);
            directed.add_edge(&iters[1], &iters[3], 6);
            directed.add_edge(&iters[1], &iters[4], 3);
            directed.add_edge(&iters[2], &iters[5], 5);
            directed.add_edge(&iters[3], &iters[4], 1);
            directed.add_edge(&iters[4], &iters[2], 9);
            directed.add_edge(&iters[4], &iters[5], 7);

            let expected_results = [0, 2, 10, 4, 5, 12];
            let result = directed.dijkstra(&iters[0]);
            assert_expected_eq_actual!(result.len(), 6);
            for (distance, iter) in result {
                let index = iter.get_index();
                assert_expected_eq_actual!(distance, expected_results[index]);
            }
        }

        #[test]
        fn connected_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let mut iters = vec![];
            for i in 0..6 {
                iters.push(undirected.add_node(&NODES[i]));
            }

            undirected.add_edge(&iters[0], &iters[1], 2);
            undirected.add_edge(&iters[0], &iters[3], 4);
            undirected.add_edge(&iters[1], &iters[2], 8);
            undirected.add_edge(&iters[1], &iters[3], 6);
            undirected.add_edge(&iters[1], &iters[4], 3);
            undirected.add_edge(&iters[2], &iters[5], 5);
            undirected.add_edge(&iters[3], &iters[4], 1);
            undirected.add_edge(&iters[4], &iters[2], 9);
            undirected.add_edge(&iters[4], &iters[5], 7);

            let expected_results = [0, 2, 10, 4, 5, 12];
            let result = undirected.dijkstra(&iters[0]);
            assert_expected_eq_actual!(result.len(), 6);
            for (distance, iter) in result {
                let index = iter.get_index();
                assert_expected_eq_actual!(distance, expected_results[index]);
            }
        }

        #[test]
        fn disconnected_directed() {
            let mut directed = Graph::new(GraphType::Directed);
            let mut iters = vec![];
            for i in 0..6 {
                iters.push(directed.add_node(&NODES[i]));
            }

            directed.add_edge(&iters[0], &iters[1], 2);
            directed.add_edge(&iters[0], &iters[3], 4);
            directed.add_edge(&iters[1], &iters[3], 6);
            directed.add_edge(&iters[1], &iters[4], 3);
            directed.add_edge(&iters[2], &iters[5], 5);
            directed.add_edge(&iters[3], &iters[4], 1);

            let expected_results = [0, 2, usize::MAX, 4, 5, usize::MAX];
            let result = directed.dijkstra(&iters[0]);
            assert_expected_eq_actual!(result.len(), 6);
            for (distance, iter) in result {
                let index = iter.get_index();
                assert_expected_eq_actual!(distance, expected_results[index]);
            }
        }

        #[test]
        fn disconnected_undirected() {
            let mut undirected = Graph::new(GraphType::Undirected);
            let mut iters = vec![];
            for i in 0..6 {
                iters.push(undirected.add_node(&NODES[i]));
            }

            undirected.add_edge(&iters[0], &iters[1], 2);
            undirected.add_edge(&iters[0], &iters[3], 4);
            undirected.add_edge(&iters[1], &iters[3], 6);
            undirected.add_edge(&iters[1], &iters[4], 3);
            undirected.add_edge(&iters[2], &iters[5], 5);
            undirected.add_edge(&iters[3], &iters[4], 1);

            let expected_results = [0, 2, usize::MAX, 4, 5, usize::MAX];
            let result = undirected.dijkstra(&iters[0]);
            assert_expected_eq_actual!(result.len(), 6);
            for (distance, iter) in result {
                let index = iter.get_index();
                assert_expected_eq_actual!(distance, expected_results[index]);
            }
        }
    }
}
