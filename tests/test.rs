extern crate graph_api;

use graph_api::{Graph,algorithm,Vertex,WeightedEdge};   
use std::collections::HashSet;

#[test]
fn graph_test() {
    let mut graph = Graph::new();
    assert_eq!(0, graph.size());

    let a = graph.add_vertex("a");
    let b = graph.add_vertex("b");
    let c = graph.add_vertex("c");
    assert_eq!(3, graph.size());

    graph.add_edge(a, b, 1);
    graph.add_edge(a, c, 2);
    assert!(graph.has_edge(a,b));
    assert!(graph.has_edge(a,c));
    assert!(!graph.has_edge(b,c));
    assert!(!graph.has_edge(c,a));

    assert!(!graph.has_edge(a,a));
    graph.add_edge(a, a, 0);
    assert!(graph.has_edge(a,a));
    assert_eq!(graph.remove_edge(a,a), 0);

    assert_eq!(graph.vertex_data(b), &"b");
    assert_eq!(graph.weight(a, b), &1);

    assert_eq!(graph.edges().collect::<HashSet<(Vertex,Vertex)>>(), 
              vec![(a,b), (a,c)].into_iter().collect());

    assert_eq!(graph.vertices()
                    .map(|v| graph.vertex_data(v))
                    .collect::<HashSet<&&str>>(), 
               vec![&"a", &"b", &"c"].into_iter().collect());
    assert_eq!(graph.remove_vertex(c), "c");
    assert_eq!(graph.vertices()
                    .map(|v| graph.vertex_data(v))
                    .collect::<HashSet<&&str>>(), 
               vec![&"a", &"b"].into_iter().collect());
    assert_eq!(graph.size(), 2);
}

#[test] 
fn minimum_spanning_tree_test() {

    // Based on http://i.imgur.com/Ly6dMt5.png. 

    let mut graph = Graph::new();
    let a = graph.add_vertex(());
    let b = graph.add_vertex(());
    let c = graph.add_vertex(());
    let d = graph.add_vertex(());
    let e = graph.add_vertex(());
    let f = graph.add_vertex(());
    let g = graph.add_vertex(());

    graph.add_bidi_edge(a, b, 4);
    graph.add_bidi_edge(a, c, 9);

    graph.add_bidi_edge(b, c, 9);
    graph.add_bidi_edge(b, d, 8);
    graph.add_bidi_edge(b, e, 10);

    graph.add_bidi_edge(c, d, 2);
    graph.add_bidi_edge(c, f, 1);

    graph.add_bidi_edge(d, e, 7);
    graph.add_bidi_edge(d, f, 9);

    graph.add_bidi_edge(e, f, 5);
    graph.add_bidi_edge(e, g, 6);

    graph.add_bidi_edge(f, g, 2);

    let result = algorithm::minimum_spanning_tree(&graph);

    // It's arbitrary whether (u,v) or (v,u) appears in the tree (because 
    // the graph is treated as undirected).
    fn has_edge(from: Vertex, to: Vertex, weight: Vertex, 
                tree: &Vec<WeightedEdge<usize>>) -> bool {
        tree.iter().any(|&(u, v, w)| 
            (u == from && v == to || u == to && v == from) && *w == weight
        ) 
    }

    match result {
        Some(tree) => {
            assert_eq!(tree.len(), graph.size() - 1);
            assert_eq!(22, tree.iter().map(|&(_, _, w)| *w).sum::<usize>());
            assert!(has_edge(a, b, 4, &tree));
            assert!(has_edge(b, d, 8, &tree));
            assert!(has_edge(d, c, 2, &tree));
            assert!(has_edge(c, f, 1, &tree));
            assert!(has_edge(f, e, 5, &tree));
            assert!(has_edge(f, g, 2, &tree));
        },
        None => assert!(false)
    };
}

#[test]
fn all_pairs_shortest_paths_test() {

    // Based on http://i.imgur.com/zxUbxrb.png.

    let mut g = Graph::<&str,isize>::new();
    let a = g.add_vertex("a");
    let b = g.add_vertex("b");
    let c = g.add_vertex("c");
    let d = g.add_vertex("d");
    let e = g.add_vertex("e");

    g.add_edge(a, b, 3);

    g.add_edge(b, d, 5);
    g.add_edge(b, c, 12);

    g.add_edge(c, a, 4);
    g.add_edge(c, d, -1);

    g.add_edge(d, a, 2);
    g.add_edge(d, b, -4);

    g.add_edge(e, c, 100);

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
        vec!(Some(101), Some(95), Some(100), Some(99),  None)

    ]);
}
