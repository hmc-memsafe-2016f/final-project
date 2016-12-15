use Graph;
use VIndex;
use std::ops::Add;

pub struct ShortestPaths<'a, E: 'a> {
    graph: &'a Graph<E>,
    dist: Vec<Vec<Option<E>>>,
    next: Vec<Vec<Option<VIndex>>>,
}

impl<'a, E: Copy> ShortestPaths<'a, E> {
    pub fn is_path(&self, from: VIndex, to: VIndex) -> bool {
        self.dist[from][to].is_some()
    }

    pub fn path_distance(&self, from: VIndex, to: VIndex) -> Option<E> {
        self.dist[from][to]
    }

    pub fn path(&self, mut from: VIndex, to: VIndex) -> Vec<(VIndex, VIndex, &'a E)> {
        let mut path = Vec::new();
        while let Some(nxt) = self.next[from][to] {
            let weight = self.graph.weight(from, nxt).unwrap();
            let e = (from, nxt, weight);
            path.push(e);
            if nxt == to {
                break;
            };
            from = nxt;
        }
        path
    }

    pub fn to_distance_matrix(self) -> Vec<Vec<Option<E>>> {
        self.dist
    }
}

pub fn floyd_warshall<'a, E>(g: &'a Graph<E>) -> ShortestPaths<'a, E>
    where E: Add<Output = E> + PartialOrd + Copy
{

    let mut dist: Vec<Vec<Option<E>>> = vec![vec![None; g.size()]; g.size()];
    let mut next = vec![vec![None; g.size()]; g.size()];

    for (from, to) in g.edges() {
        dist[from][to] = g.weight(from, to).map(|index| *index);
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
