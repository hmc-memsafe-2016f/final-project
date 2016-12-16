extern crate mjgraph;
extern crate union_find;
use mjgraph::Graph;
use mjgraph::Edge;
use mjgraph::Vertex;
use union_find::QuickFindUf;
use std::hash::Hash;

pub fn kruskal<V: Copy + Clone + Eq + Hash,
    E: Ord + Copy + Clone + Default>(g: mjgraph::Graph<V, E>) -> Vec<Edge<E>> {
    let vertices = union_find::QuickFindUf::<&Vertex<V>>::new(0);
    let mut tree = Vec::new();
    let mut edges = Vec::<Edge<E>>::new();
    for v in g.vertices() {
        for e in v.edges() {
            edges.push(*e);
        }
    }
    edges.sort();
    for ref v in g.vertices() {
        vertices.insert(v);
    }
    for e in edges {
        let (u, v) = g.endpoints(e);
        let id1 = vertices.find(u);
        let id2 = vertices.find(v);
        if id1 != id2 {
            tree.push(e);
            vertices.union(id1, id2);
        }
    }
    tree
}

fn main() {
}

