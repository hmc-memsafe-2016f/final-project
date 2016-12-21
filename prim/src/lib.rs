/*
BROKEN (out of index error and GOD WHY DOESNT RUST HAVE GOOD BACKTRACE that doesn't just give you STACK POINTERS).
*/

extern crate graph;

use graph::Graph;
use std::collections::HashSet;



pub fn mst<V>(g: Graph<V, usize>) -> Vec<usize>{
	//let mut heap = BinaryHeap::new(); // heap of vertices to visit 
	//(couldn't get to work, so asymptotic will be n^2m instead of nlogn m, namely, doesn't support modifying internal values)
	let size = g.get_vertices().len();
	let mut dists = vec![usize::max_value(); size]; // vertex to distance
	let mut nears = vec![usize::max_value(); size]; // vertex to distance
	let mut set = HashSet::new(); // set of included vertices
	let mut tree = Vec::new(); // vec of indices of edges in mst


	dists[0] = 0;
	set.insert(0);
	while set.len() != size{
		let mut min_dist = usize::max_value();
		let mut closest = 0;
		for v in 0..size{ // find closest vertex to tree (could be done in O(1) with minheap)
			if !set.contains(&v) && min_dist > dists[v]{
				min_dist = dists[v];
				closest = v;
			}
		}
		set.insert(closest);
		tree.push(*Option::unwrap(g.get_edge_ind(nears[closest], closest)));

		for v in g.get_neighbors(closest){
			if dists[*v] > *Option::unwrap(g.get_edge(closest,*v)) {
				nears[*v] = closest;
			}
		}	
	}
	tree
}