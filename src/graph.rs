use std::vec::IntoIter;

pub struct Graph<E> {
    matrix: Vec<Vec<Option<E>>>,
    size: usize,
}

pub type VIndex = usize;

pub type Edge = (VIndex, VIndex);

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

    pub fn edges(&self) -> Edges {
        Edges::new(self)
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

pub struct Edges {
    iter: IntoIter<Edge>,
}

impl Edges {
    fn new<E>(graph: &Graph<E>) -> Edges {
        let mut edges = Vec::new();
        let mut from = 0;
        let mut to = 0;
        while from < graph.size() {
            if let Some(_) = graph.matrix[from][to] {
                edges.push((from, to));
            }
            to += 1;
            if to >= graph.size {
                from += 1;
                to = 0;
            }
        }
        Edges { iter: edges.into_iter() }
    }
}

impl Iterator for Edges {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
