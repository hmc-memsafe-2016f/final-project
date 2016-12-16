extern crate graph;

use graph::Graph;

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

// This attribute identifies this function as a unit test
#[test]
fn new() {
    let g = Graph::<i64, i64>::new();
}

// This attribute identifies this function as a unit test
#[test]
fn createnode() {
    let mut g = Graph::<i64, i64>::new();
    let anode = g.create_node(10);
    assert!(g.map_node(&anode, |x| {*x==10}).unwrap(), true)
}

// This attribute identifies this function as a unit test
#[test]
fn createnode() {
    let mut g = Graph::<i64, i64>::new();
    let anode = g.create_node(10);
    assert!(g.map_node(&anode, |x| {*x==10}).unwrap(), true)
}

fn getnodes() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    let c_node = g.create_node(10);
    g.create_edge(a_node, b_node, 5);
    g.create_edge(a_node, c_node, 5);

    for node in g.nodes()
    {
    	assert!(g.map_node(&node, |x| {*x==10}).unwrap(), true)
    }
}

