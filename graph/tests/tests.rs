extern crate graph;

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

    #[allow(unused_variables)]
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
    fn djikstra_cycle() {
        let mut g = Graph::<u8>::new();
        let a = g.add_node(100);
        let b = g.add_node(50);
        let c = g.add_node(1);
        g.add_edge(a, b, 1);
        g.add_edge(b, c, 1);
        g.add_edge(c, a, 1);
        let y = g.add_node(8);
        let z = g.add_node(88);
        g.add_edge(y,a,1);
        g.add_edge(c,z,1);
        assert_expected_eq_actual!(djikstra(&g, y, z), 4);
    }

    #[test]
    fn djikstra_cycle_unreachable() {
        let mut g = Graph::<u8>::new();
        let a = g.add_node(100);
        let b = g.add_node(50);
        let c = g.add_node(1);
        g.add_edge(a, b, 1);
        g.add_edge(b, c, 1);
        g.add_edge(c, a, 1);
        let y = g.add_node(8);
        let z = g.add_node(88);
        g.add_edge(y,a,1);
        assert_expected_eq_actual!(djikstra(&g, y, z), usize::max_value());
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
    fn topo_sort_complex() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(1);
        let b = x.add_node(2);
        let c = x.add_node(3);
        let d = x.add_node(4);
        let e = x.add_node(5);
        x.add_edge(a, b, 1);
        x.add_edge(b, c, 1);
        x.add_edge(b, d, 1);
        x.add_edge(c, d, 1);
        x.add_edge(c, e, 1);
        x.add_edge(e, d, 1);
        let sorted = topo_sort(&x).unwrap();
        assert_expected_eq_actual!(sorted.len(), 5);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[0]), 1);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[1]), 2);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[2]), 3);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[3]), 5);
        assert_expected_eq_actual!(*x.get_vertex_data(sorted[4]), 4);
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

    #[test]
    fn topo_sort_cycle2() {
        let mut x = Graph::<u8>::new();
        let a = x.add_node(100);
        let b = x.add_node(50);
        let c = x.add_node(1);
        let d = x.add_node(2);
        x.add_edge(d, a, 5);
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