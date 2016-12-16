use {Graph, Vertex, WeightedEdge};
use std::ops::Add;

/// Structure that holds the shortest path between any pair of vertices in a
/// graph.
pub struct ShortestPaths<'a, V, E>
    where V: 'a,
          E: 'a
{
    graph: &'a Graph<V, E>,
    dist: Vec<Vec<Option<E>>>,
    next: Vec<Vec<Option<Vertex>>>,
}

impl<'a, V, E> ShortestPaths<'a, V, E>
    where E: Copy
{
    /// Whether there is a path from the `from` vertex to the `to` vertex.
    pub fn is_path(&self, from: Vertex, to: Vertex) -> bool {
        self.dist[from][to].is_some()
    }

    /// The sum of the edge weights on the shortest path from the `from` vertex
    /// to the `to` vertex.
    pub fn path_distance(&self, from: Vertex, to: Vertex) -> Option<E> {
        self.dist[from][to]
    }

    /// Returns an ordered list of weighted edges on the shortest path from the
    /// `from` vertex to the `to` vertex.
    ///
    /// The list is empty if there is no path between the vertices.
    pub fn path(&self, from: Vertex, to: Vertex) -> Vec<WeightedEdge<'a, E>> {
        let mut path = Vec::new();
        let mut src = from;
        while let Some(nxt) = self.next[src][to] {
            let weight = self.graph.weight(src, nxt);
            let e = (src, nxt, weight);
            path.push(e);
            if nxt == to {
                break;
            };
            src = nxt;
        }
        path
    }

    /// Returns `matrix` such that `matrix[u][v]` is `None` if there is no path
    /// from `Vertex` `u` to `Vertex` `v` and `Some(d)` if the distance of the
    /// shortest path is `d`.
    pub fn to_distance_matrix(self) -> Vec<Vec<Option<E>>> {
        self.dist
    }
}

/// Uses the Floyd-Warshall algorithm to compute the shortest path between
/// every pair of vertices in the graph.
///
/// This algorithm may return incorrect results if a negative cycle is present
/// in the graph.
///
/// Note that we do not assume there to be reflexive edges on the vertices.
/// In other words, this function will report no path from a vertex to itself
/// unless a edge is specifically added using `add_vertex`.
///
/// The type bounds on `E` (edge weight) should allow for any reasonable
/// numeric type.
pub fn floyd_warshall<'a, V, E>(g: &'a Graph<V, E>) -> ShortestPaths<'a, V, E>
    where E: Add<Output = E> + PartialOrd + Copy
{

    let mut dist: Vec<Vec<Option<E>>> = vec![vec![None; g.size()]; g.size()];
    let mut next = vec![vec![None; g.size()]; g.size()];

    for (from, to) in g.edges() {
        dist[from][to] = Some(*g.weight(from, to));
        next[from][to] = Some(to);
    }
    for k in 0..g.size() {
        for i in 0..g.size() {
            for j in 0..g.size() {
                if greater(dist[i][j], plus(dist[i][k], dist[k][j])) {
                    dist[i][j] = plus(dist[i][k], dist[k][j]);
                    next[i][j] = next[i][k]
                }
            }
        }
    }
    for i in 0..g.size() {
        dist[i][i] = None;
        next[i][i] = None;
    }
    ShortestPaths {
        graph: g,
        dist: dist,
        next: next,
    }
}


fn plus<N>(a: Option<N>, b: Option<N>) -> Option<N>
    where N: Add<Output = N>
{
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (_, _) => None,
    }
}

fn greater<N: PartialOrd>(a: Option<N>, b: Option<N>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a > b,
        (None, Some(_)) => true,
        (_, None) => false,
    }
}
