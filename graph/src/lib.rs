use std::collections::HashMap;

pub struct Graph<V,E>{
	vertices : Vec<V>,
	edges : Vec<HashMap<usize, usize>>, // each vertex has a map from its neighbors
	pub edge_data : Vec<E>,                 // to the corresponding edge data
}


impl <V,E> Graph<V,E> {
	pub fn new() -> Graph<V,E>{
		Graph{
			vertices : Vec::new(),
			edges : Vec::new(),
			edge_data : Vec::new(),
		}
	}

	pub fn add_vertex(&mut self, vertex: V) -> usize{
		self.vertices.push(vertex);
		self.edges.push(HashMap::new());
		self.vertices.len()-1
	}

	pub fn add_dir_edge(&mut self, data: E, u: usize, v: usize){
		self.edges[u].insert(v, self.edge_data.len());
		self.edge_data.push(data);
	}

	pub fn add_edge(&mut self, data: E, u: usize, v: usize){
		self.edges[u].insert(v, self.edge_data.len());
		self.edges[v].insert(from, self.edge_data.len());
		self.edge_data.push(data);
	}

	pub fn get_vertices(&self) -> &Vec<V>{
		&self.vertices
	}

	pub fn get_vertex(&self, v: usize) -> &V{
		&self.vertices[v]
	}

	pub fn get_edge_ind(&self, u: usize, v: usize) -> Option<&usize>{
		self.edges[u].get(&v)
	}

	pub fn get_edge(&self, u: usize, v: usize) -> Option<&E>{
		self.edges[u].get(&v).map(|e| &self.edge_data[*e])
	}

	pub fn get_neighbors(&self, u: usize) -> Vec<(&usize)>{
		let mut neighbors = Vec::new();
		for (v, _) in &self.edges[u] {
			neighbors.push(v);
		}
		neighbors
	}
}