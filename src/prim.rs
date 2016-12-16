use {Graph, WeightedEdge, Vertex};
use std::collections::HashSet;

/// Uses Prim's algorithm to compute a minimum spanning tree (MST) for a graph.
/// Returns a set of weighted edges in the MST.
///
/// Note: this function treats the graph as if it were undirected, and whether
/// the output set contains edge `(u,v)` or `(v,u)` is arbitrary.
pub fn prim<'a, V, E>(g: &'a Graph<V, E>) -> Option<Vec<WeightedEdge<'a, E>>>
    where E: Ord
{
    let mut vertices = HashSet::new();
    let mut tree = Vec::new();
    match g.vertices().next() {
        Some(v) => vertices.insert(v),
        None => return Some(tree),
    };
    let mut edges = g.edges()
        .map(|(from, to)| (from, to, g.weight(from, to)))
        .collect::<Vec<WeightedEdge<'a, E>>>();
    edges.sort_by_key(get_weight);
    while let Some(&(from, to, weight)) = edges.iter()
        .filter(|edge| will_expand_tree(&vertices, edge))
        .next() {
        vertices.insert(from);
        vertices.insert(to);
        tree.push((from, to, weight));
    }
    if g.size() == vertices.len() {
        Some(tree)
    } else {
        None
    }
}

fn will_expand_tree<'a, E>(vs: &HashSet<Vertex>, e: &WeightedEdge<'a, E>) -> bool {
    match e {
        &(ref from, ref to, _) => {
            (vs.contains(from) || vs.contains(to)) && !(vs.contains(from) && vs.contains(to))
        }
    }
}

fn get_weight<'a, E>(edge: &WeightedEdge<'a, E>) -> &'a E {
    match edge {
        &(_, _, w) => w,
    }
}
