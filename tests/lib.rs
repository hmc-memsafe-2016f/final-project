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