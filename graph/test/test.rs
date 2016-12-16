use graph::Graph;

#[derive(Debug)]
struct V{ data: usize }
#[derive(Debug)]
struct E{ data: usize }

fn main() {
    let mut g = Graph::new();
    let v1 = g.add_vertex(V{data: 4});
    let v2 = g.add_vertex(V{data: 6});
    g.add_edge(v2, v1, E{data: 7});

    println!("{:?}", g);
    println!("{:?}", g.get_edge(v1, v2));
    println!("{:?}", g.get_edge(v2, v1));
    println!("{:?}", g.neighbors(v1).collect::<Vec<_>>());
    println!("{:?}", g.neighbors(v2).collect::<Vec<_>>());
    println!("{:?}", g.incident_edges(v1).collect::<Vec<_>>());
    println!("{:?}", g.incident_edges(v2).collect::<Vec<_>>());
}
