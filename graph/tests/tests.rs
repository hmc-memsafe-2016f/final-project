extern crate graph;

pub use graph::{Graph,djikstra,topo_sort};

// This macro is an assertion with nicely formatted failure output
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

mod tests {
    use graph::Graph;

    #[test]
    fn new_exists() {
        let x = Graph::<u8>::new();
    }

    #[test]
    fn get_all_nodes() {
        let x = Graph::<u8>::new();
        let all_nodes = x.get_all_nodes();
        assert_expected_eq_actual!(all_nodes.len(), 0);
    }

    #[test]
    fn add_node() {
        let mut x = Graph::<u8>::new();
        x.add_node(5);
        assert_expected_eq_actual!(x.get_all_nodes().len(), 1);
    }

    #[test]
    fn add_edge_get_neighbors() {
        let mut x = Graph::<u8>::new();
        let one = x.add_node(1);
        let two = x.add_node(2);
        assert_expected_eq_actual!(x.get_neighbors(one).len(), 0);
        assert_expected_eq_actual!(x.get_neighbors(two).len(), 0);
        x.add_edge(one, two, 1);
        assert_expected_eq_actual!(x.get_neighbors(one).len(), 1);
        assert_expected_eq_actual!(x.get_neighbors(two).len(), 0);
    }

}