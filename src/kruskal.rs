use mjgraph;

pub fn kruskal(g: mjgraph::Graph<V, E: Ord>) -> Vec<Edge<E>> {
    let vertices = union_find::QuickFindUf<&Vertex<V, E>>::new();
    let mut tree = Vec::new();
    let mut edges = Vec<Edge<E>>::new();
    for ref e in g.edges() {
        edges.push(e);
    }
    edges.sort();
    for ref v in g.vertices() {
        vertices.insert(v);
    }
    for e in edges {
        let (u, v) = e.endpoints();
        let id1 = vertices.find(u);
        let id2 = vertices.find(v);
        if id1 != id2 {
            tree.push(e);
            vertices.union(id1, id2);
        }
    }
}
