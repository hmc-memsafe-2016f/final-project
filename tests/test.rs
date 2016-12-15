extern crate graph_api;

use graph_api::{Graph,algorithm};   

#[test]
fn basic_test() {
    let mut graph = Graph::new();
    let a = graph.add_vertex();
    let b = graph.add_vertex();
    let c = graph.add_vertex();
    graph.add_edge(a, b, 0);
    graph.add_edge(a, c, 1);
    assert!(graph.has_edge(a,b));
    assert!(graph.has_edge(a,c));
    assert!(!graph.has_edge(b,c));
    assert!(!graph.has_edge(c,a));
    
}

#[test]
fn all_pairs_shortest_paths_test() {
    let mut g = Graph::<isize>::new();
    let (a,b,c,d,e) = (g.add_vertex(),g.add_vertex(),g.add_vertex(),g.add_vertex(),g.add_vertex());

    g.add_edge(a, b, 3);

    g.add_edge(b, d, 5);
    g.add_edge(b, c, 12);

    g.add_edge(c, a, 4);
    g.add_edge(c, d, -1);

    g.add_edge(d, a, 2);
    g.add_edge(d, b, -4);

    g.add_edge(e, d, 100);

    let result = algorithm::all_pairs_shortest_path(&g);

    assert!(result.is_path(a, b));
    assert!(result.is_path(e, c));
    assert!(!result.is_path(c, c));
    assert!(!result.is_path(c, e));

    assert_eq!(result.path_distance(a, c), Some(15));
    assert_eq!(result.path_distance(c, e), None);

    assert_eq!(result.path(c, b), vec![
        (c, d, &-1),
        (d, b, &-4)
    ]);

    let result_matrix = result.to_distance_matrix();

    assert_eq!(result_matrix, vec![
        vec![None,      Some(3),  Some(15),  Some(8),   None],
        vec![Some(7),   None,     Some(12),  Some(5),   None],
        vec![Some(1),   Some(-5), None,      Some(-1),  None],
        vec![Some(2),   Some(-4), Some(8),   None,      None],
        vec!(Some(102), Some(96), Some(108), Some(100), None)

    ]);}
