extern crate graph_api;

use graph_api::{Graph,Vertex,floyd_warshall};   

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
    let mut g = Graph::<usize>::new();
    let (a,b,c,d) = (g.add_vertex(),g.add_vertex(),g.add_vertex(),g.add_vertex());

    g.add_edge(a, b, 3);

    g.add_edge(b, d, 5);
    g.add_edge(b, c, 12);

    g.add_edge(c, a, 4);
    g.add_edge(c, d, -1);

    g.add_edge(d, a, 2);
    g.add_edge(d, b, -4);

    let apsp = floyd_warshall(&g).to_distance_matrix();
    for row in apsp {
        for distance in apsp {
            print!("{} ", distance);
        }
        println!("");
    }
}
