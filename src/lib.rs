extern crate num;

use std::ops::Add;
use num::{Num,Zero};

pub struct Graph<E> {
    matrix: Vec<Vec<Option<E>>>,
    pub vertices: usize,
}

pub type Vertex = usize;

#[derive(Debug)]
pub struct Edge<E> {
    pub from: Vertex,
    pub to: Vertex,
    pub weight: E,
}

impl <E> Graph<E> {

    pub fn new() -> Self {
        Graph { matrix: Vec::new(), vertices: 0 }
    }

    pub fn add_vertex(&mut self) -> Vertex {
        self.vertices += 1;
        for row in &mut self.matrix {
            row.push(None)
        }
        self.matrix.push(nones(self.vertices));
        self.vertices - 1
    }

    pub fn add_edge(&mut self, from: Vertex, to: Vertex, weight: E) {
        self.matrix[from][to] = Some(weight);
    }

    pub fn has_edge(&self, from: Vertex, to: Vertex) -> bool {
        self.matrix[from][to].is_some()
    }

    pub fn weight(&self, from: Vertex, to: Vertex) -> &Option<E> {
        &self.matrix[from][to]
    }

    pub fn edges<'a>(&'a self) -> Edges<'a,E> {
        Edges { graph: &self, from: 0, to: usize::max_value() }
    }

}

pub struct Edges<'a,E:'a> {
    graph: &'a Graph<E>,
    from: Vertex,
    to: Vertex,
}

impl <'a,E> Iterator for Edges<'a,E> {
    type Item = Edge<&'a E>;
    fn next(&mut self) -> Option<Edge<&'a E>> {
        self.to = self.to.wrapping_add(1);
        if self.to >= self.graph.vertices {
            self.from += 1;
            self.to = 0;
        }
        if self.from >= self.graph.vertices {
            return None;
        }
        match self.graph.matrix[self.from][self.to] {
            Some(ref weight) => {
                Some(Edge {
                    from: self.from,
                    to: self.to,
                    weight: weight
                })
            }
            None => self.next()
        }
    }
}


fn plus<N: Add<Output=N>>(a: Option<N>, b: Option<N>) -> Option<N> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a+b),
        (_,       _      ) => None
    }
}

fn greater<N: PartialOrd>(a: Option<N>, b: Option<N>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a > b,
        (None,    Some(_)) => true,
        (_,       None   ) => false,
    }
}

pub struct ShortestPaths<'a,E:'a> {
    graph: &'a Graph<E>,
    dist: Vec<Vec<Option<&'a E>>>,
    next: Vec<Vec<Option<Vertex>>>,
}

impl <'a,E:Num> ShortestPaths<'a,E> where &'a E: Num {

    pub fn is_path(&self, from: Vertex, to: Vertex) -> bool {
        self.dist[from][to].is_some()
    }

    pub fn path_distance(&self, from: Vertex, to: Vertex) -> Option<&'a E> {
        self.dist[from][to]
    }

    pub fn path(&self, mut from: Vertex, to: Vertex) -> Vec<Edge<&'a E>> {
        let mut path = Vec::new();
        while let Some(nxt) = self.next[from][to] {
            let weight = self.graph.weight(from, nxt)
                                   .as_ref()
                                   .unwrap_or(Zero::zero());
            let e = Edge { from: from, to: nxt, weight: weight};
            path.push(e);
            if nxt == to {
                break
            };
            from = nxt;
        }
        path
    }

    pub fn to_distance_matrix(self) -> Vec<Vec<Option<&'a E>>> {
        self.dist
    }

}

pub fn floyd_warshall<'a,E>(g: &'a Graph<E>) -> ShortestPaths<'a,E> 
where &'a E: Num + PartialOrd {
    let mut dist = vec![vec![None; g.vertices]; g.vertices];
    let mut next = vec![vec![None; g.vertices]; g.vertices];
    for i in 0..g.vertices {
        dist[i][i] = Some(Zero::zero());
        next[i][i] = Some(i);
    }
    for edge in g.edges() {
        dist[edge.from][edge.to] = Some(edge.weight);
        next[edge.from][edge.to] = Some(edge.to);
    }
    for k in 0..g.vertices {
        for i in 0..g.vertices {
            for j in 0..g.vertices {
                if greater(dist[i][j], plus(dist[i][k], dist[k][j])) {
                    dist[i][j] = plus(dist[i][k], dist[k][j]);
                    next[i][j] = next[i][k]
                }
            }
        }
    }
    ShortestPaths { graph: g, dist: dist, next: next }
}


fn nones<T>(n: usize) -> Vec<Option<T>> {
    if n == 0 {
        Vec::new()
    } else {
        let mut v = nones(n-1);
        v.push(None);
        v
    }
}