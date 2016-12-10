extern crate memsafe_graph;

pub use memsafe_graph::Graph;

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
    mod vf2 {
        use super::super::Graph;

    }

    mod dijkstra {
        use super::super::Graph;

    }
}
