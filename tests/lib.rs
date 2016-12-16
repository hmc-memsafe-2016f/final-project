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
    assert_expected_eq_actual!(g.map_node(&anode, |x| {*x==10}).unwrap(), true)
}

// This attribute identifies this function as a unit test
#[test]
fn getedge() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    let c_node = g.create_node(10);
    let d_node = g.create_node(10);
    g.create_edge(&a_node, &b_node, 5);
    g.create_edge(&a_node, &c_node, 6);
    g.create_edge(&a_node, &d_node, 7);
    assert_expected_eq_actual!(g.get_edge(&a_node, &c_node).unwrap(), 6);
    assert_expected_eq_actual!(g.get_edge(&a_node, &d_node).unwrap(), 7)
}


// This attribute identifies this function as a unit test
#[test]
fn getneigbors() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    let c_node = g.create_node(10);
    let d_node = g.create_node(10);
    g.create_edge(&a_node, &b_node, 5);
    g.create_edge(&a_node, &c_node, 5);
    g.create_edge(&a_node, &d_node, 5);
    assert_expected_eq_actual!(g.neighbors(&a_node).unwrap().iter().count(), 3)
}


// This attribute identifies this function as a unit test
#[test]
fn make_edge_undirected() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    g.create_edge_undirected(&a_node, &b_node, 5);
    assert_expected_eq_actual!(g.get_edge(&a_node, &b_node).unwrap(), 5);
    assert_expected_eq_actual!(g.get_edge(&b_node, &a_node).unwrap(), 5)
}


// This attribute identifies this function as a unit test
#[test]
fn delete_edge_undirected() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    g.create_edge_undirected(&a_node, &b_node, 5);
    g.delete_edge_undirected(&a_node, &b_node);
    assert!(g.neighbors(&a_node).unwrap().is_empty());
    assert!(g.neighbors(&b_node).unwrap().is_empty())
}


// This attribute identifies this function as a unit test
#[test]
fn delete_node() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    g.create_edge_undirected(&a_node, &b_node, 5);
    g.delete_node(a_node);
    assert!(g.neighbors(&b_node).unwrap().is_empty())
}


// This attribute identifies this function as a unit test
#[test]
fn replace_edge() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    g.create_edge_undirected(&a_node, &b_node, 5);
    g.create_edge(&b_node, &a_node, 10);
    assert_expected_eq_actual!(g.get_edge(&b_node, &a_node).unwrap(), 10)
}


// This attribute identifies this function as a unit test
#[test]
fn replace_node() {
    let mut g = Graph::<i64, i64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    let old_a = g.replace_node(&a_node, 20).unwrap();
    assert_expected_eq_actual!(old_a, 10);
    assert_expected_eq_actual!(g.map_node(&a_node, |x| {*x==20}).unwrap(), true)
}


// This attribute identifies this function as a unit test
#[test]
fn map_node_mut() {
    let mut g = Graph::<u64, u64>::new();
    let a_node = g.create_node(10);
    g.map_node_mut(&a_node, |x| {
        let ret = *x;
        *x=2*(*x);
        ret
    });
    assert_expected_eq_actual!(g.map_node(&a_node, |x| {*x==20}).unwrap(), true)
}


// This attribute identifies this function as a unit test
#[test]
fn map_edge_mut() {
    let mut g = Graph::<u64, u64>::new();
    let a_node = g.create_node(10);
    let b_node = g.create_node(10);
    g.create_edge_undirected(&a_node, &b_node, 5);
    g.map_edge_mut(&a_node, &b_node, |x| {
        let ret = *x;
        *x=2*(*x);
        ret
    });
    assert_expected_eq_actual!(g.get_edge(&a_node, &b_node).unwrap(), 10)
}