pub struct Graph<E> {
    matrix: Vec<Vec<Option<E>>>,
    vertices: usize,
}

pub type Vertex = usize;

#[derive(Debug)]
pub struct Edge<'a, E: 'a> {
    from: Vertex,
    to: Vertex,
    weight: &'a E,
}

impl <'a, E: 'a> Edge<'a, E> {
    pub fn from(&self) -> Vertex { self.from }
    pub fn to(&self) -> Vertex { self.to }
    pub fn weight(&self) -> &'a E { self.weight }
}

impl <E> Graph<E> {

    pub fn new() -> Self {
        Graph { matrix: Vec::new(), vertices: 0 }
    }

    pub fn size(&self) -> usize {
        self.vertices
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

    pub fn weight(&self, from: Vertex, to: Vertex) -> Option<&E> {
        self.matrix[from][to].as_ref()
    }

    pub fn vertices<'a>(&'a self) -> Vertices<'a,E> {
        Vertices { graph: &self, current: 0 }
    }

    pub fn edges<'a>(&'a self) -> Edges<'a,E> {
        Edges { graph: &self, from: 0, to: usize::max_value() }
    }

    pub fn remove_vertex(mut self, index: Vertex) -> Graph<E> {
        self.matrix.remove(index);
        for row in &mut self.matrix {
            row.remove(index);
        };
        self.vertices -= 1;
        self
    }

    pub fn remove_edge(mut self, from: Vertex, to: Vertex) -> Graph<E> {
        self.matrix[from][to] = None;
        self
    }

}

pub struct Vertices<'a,E:'a> {
    graph: &'a Graph<E>,
    current: usize,
}

impl <'a,E> Iterator for Vertices<'a,E> {
    type Item = Vertex;
    fn next(&mut self) -> Option<Vertex> {
        if self.current >= self.graph.vertices {
            None
        } else {
            self.current += 1;
            Some(self.current - 1)
        }
    }
}

pub struct Edges<'a, E: 'a> {
    graph: &'a Graph<E>,
    from: Vertex,
    to: Vertex,
}

impl <'a,E> Iterator for Edges<'a,E> {
    type Item = Edge<'a,E>;
    fn next(&mut self) -> Option<Edge<'a,E>> {
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

fn nones<T>(n: usize) -> Vec<Option<T>> {
    if n == 0 {
        Vec::new()
    } else {
        let mut v = nones(n-1);
        v.push(None);
        v
    }
}