extern crate graph_api;

use graph_api::{Graph,Vertex};   

fn add_vertices(g: &mut Graph, num: usize) -> Vec<Vertex> {
    if num == 0 {
        Vec::new()
    } else {
        let mut v = add_vertices(g, num - 1);
        v.push(g.add_vertex());
        v
    }
}

#[test]
fn test() {
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
    for e in graph.edges() {
        println!("{:?}", e)
    }
}

// #[test]
// fn kruskal() {
//     let mut g = Graph::new();
//     v = add_vertices(g, 10);

//     graph.add_edge(v[0], v[3], 6);
//     graph.add_edge(v[0], v[1], 3); // MST
//     graph.add_edge(v[0], v[9], 9);

//     graph.add_edge(v[0], v[3], 6);
   

// }
