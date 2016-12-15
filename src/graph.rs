pub struct Graph<E> {
    matrix: Vec<Vec<Option<E>>>,
    size: usize,
}

pub type VIndex = usize;

pub struct Edge<'a, E: 'a> {
    from: VIndex,
    to: VIndex,
    weight: &'a E,
}

impl<'a, E: 'a> Edge<'a, E> {
    pub fn from(&self) -> VIndex {
        self.from
    }
    pub fn to(&self) -> VIndex {
        self.to
    }
    pub fn weight(&self) -> &'a E {
        self.weight
    }
}

impl<E> Graph<E> {

    pub fn new() -> Self {
        Graph {
            matrix: Vec::new(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_vertex(&mut self) -> VIndex {
        self.size += 1;
        for row in &mut self.matrix {
            row.push(None)
        }
        let mut row = Vec::new();
        for _ in 0..self.size {
            row.push(None)
        }
        self.matrix.push(row);
        self.size - 1
    }

    pub fn add_edge(&mut self, from: VIndex, to: VIndex, weight: E) {
        self.matrix[from][to] = Some(weight);
    }

    pub fn has_edge(&self, from: VIndex, to: VIndex) -> bool {
        self.matrix[from][to].is_some()
    }

    pub fn weight(&self, from: VIndex, to: VIndex) -> Option<&E> {
        self.matrix[from][to].as_ref()
    }

    pub fn edges<'a>(&'a self) -> Edges<'a, E> {
        Edges {
            graph: &self,
            from: 0,
            to: usize::max_value(),
        }
    }

    pub fn remove_vertex(mut self, index: VIndex) -> Graph<E> {
        self.matrix.remove(index);
        for row in &mut self.matrix {
            row.remove(index);
        }
        self.size -= 1;
        self
    }

    pub fn remove_edge(mut self, from: VIndex, to: VIndex) -> Graph<E> {
        self.matrix[from][to] = None;
        self
    }

}

pub struct Edges<'a, E: 'a> {
    graph: &'a Graph<E>,
    from: VIndex,
    to: VIndex,
}

impl<'a, E> Iterator for Edges<'a, E> {
    type Item = Edge<'a, E>;
    fn next(&mut self) -> Option<Edge<'a, E>> {
        self.to = self.to.wrapping_add(1);
        if self.to >= self.graph.size {
            self.from += 1;
            self.to = 0;
        }
        if self.from >= self.graph.size {
            return None;
        }
        match self.graph.matrix[self.from][self.to] {
            Some(ref weight) => {
                Some(Edge {
                    from: self.from,
                    to: self.to,
                    weight: weight,
                })
            }
            None => self.next(),
        }
    }
}
