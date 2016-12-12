extern crate graph;

pub use graph::{Graph,djikstra,topo_sort};

// This macro is an assertion with nicely formatted failure output
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nActual `{:?}` is not equal to Expected `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

mod tests {
    use graph::Graph;
    use graph::djikstra;
    use graph::topo_sort;

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
    fn get_vertex_data() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(5);
        assert_expected_eq_actual!(*x.get_vertex_data(a), 5);
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

    #[test]
    fn djikstra_trivial() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(1);
        let b = x.add_node(2);
        x.add_edge(a, b, 8);
        assert_expected_eq_actual!(djikstra(&x, a, b), 8);
    }

    #[test]
    fn djikstra_simple() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(1);
        let b = x.add_node(2);
        let c = x.add_node(50);
        x.add_edge(a, b, 8);
        x.add_edge(a, c, 15);
        x.add_edge(b, c, 6);
        assert_expected_eq_actual!(djikstra(&x, a, c), 14);
    }

    #[test]
    fn djikstra_simple2() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(1);
        let b = x.add_node(2);
        let c = x.add_node(50);
        x.add_edge(a, b, 8);
        x.add_edge(a, c, 15);
        x.add_edge(b, c, 8);
        assert_expected_eq_actual!(djikstra(&x, a, c), 15);
    }

    #[test]
    fn djikstra_unreachable() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(1);
        let b = x.add_node(2);
        assert_expected_eq_actual!(djikstra(&x, a, b), usize::max_value());
    }

    #[test]
    fn topo_sort_basic() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(100);
        let b = x.add_node(50);
        x.add_edge(a, b, 1);
        let sorted = topo_sort(&x).unwrap();
        assert_expected_eq_actual!(sorted.len(), 2);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[0]), 100);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[1]), 50);
    }

    #[test]
    fn topo_sort_cycle() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(100);
        let b = x.add_node(50);
        let c = x.add_node(1);
        x.add_edge(a, b, 1);
        x.add_edge(b, c, 1);
        x.add_edge(c, a, 1);
        let sorted = topo_sort(&x);
        match sorted {
            Some(_) => assert!(false, "should not be able to topo_sort this graph"),
            None => ()
        }
    }

}