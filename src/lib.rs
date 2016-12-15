pub struct Graph {
    matrix: Vec<Vec<Option<isize>>>,
    pub vertices: usize,
}

pub type Vertex = usize;

#[derive(Debug)]
pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    pub weight: isize,
}

impl Graph {

    pub fn new() -> Self {
        Graph { matrix: Vec::new(), vertices: 0 }
    }

    pub fn add_vertex(&mut self) -> Vertex {
        self.vertices += 1;
        for row in &mut self.matrix {
            row.push(None)
        }
        self.matrix.push(vec![None; self.vertices]);
        self.vertices - 1
    }

    pub fn add_edge(&mut self, from: Vertex, to: Vertex, weight: isize) {
        self.matrix[from][to] = Some(weight);
    }

    pub fn has_edge(&self, from: Vertex, to: Vertex) -> bool {
        self.matrix[from][to].is_some()
    }

    pub fn weight(&self, from: Vertex, to: Vertex) -> Option<isize> {
        self.matrix[from][to]
    }

    pub fn edges<'a>(&'a self) -> Edges<'a> {
        Edges { graph: &self, from: 0, to: usize::max_value() }
    }

}

pub struct Edges<'a> {
    graph: &'a Graph,
    from: Vertex,
    to: Vertex,
}

impl <'a> Iterator for Edges<'a> {
    type Item = Edge;
    fn next(&mut self) -> Option<Edge> {
        self.to = self.to.wrapping_add(1);
        if self.to >= self.graph.vertices {
            self.from += 1;
            self.to = 0;
        }
        if self.from >= self.graph.vertices {
            return None;
        }
        match self.graph.matrix[self.from][self.to] {
            Some(weight) => {
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


fn plus(a: Option<isize>, b: Option<isize>) -> Option<isize> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a+b),
        (_,       _      ) => None
    }
}

fn greater(a: Option<isize>, b: Option<isize>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a > b,
        (None,    Some(_)) => true,
        (_,       None   ) => false,
    }
}

pub struct ShortestPaths<'a> {
    graph: &'a Graph,
    dist: Vec<Vec<Option<isize>>>,
    next: Vec<Vec<Option<Vertex>>>,
}

impl <'a> ShortestPaths<'a> {

    pub fn is_path(&self, from: Vertex, to: Vertex) -> bool {
        self.dist[from][to].is_some()
    }

    pub fn path_length(&self, from: Vertex, to: Vertex) -> Option<isize> {
        self.dist[from][to]
    }

    pub fn path(&self, mut from: Vertex, to: Vertex) -> Vec<Edge> {
        let mut path = Vec::new();
        while let Some(nxt) = self.next[from][to] {
            let e = Edge { from: from, 
                           to: nxt, 
                           weight: self.graph.weight(from, nxt).unwrap_or(0) };
            path.push(e);
            if nxt == to {
                break
            };
            from = nxt;
        }
        path
    }

}

pub fn floyd_warshall<'a>(g: &'a Graph) -> ShortestPaths<'a> {
    let mut dist = vec![vec![None; g.vertices]; g.vertices];
    let mut next = vec![vec![None; g.vertices]; g.vertices];
    for i in 0..g.vertices {
        dist[i][i] = Some(0);
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