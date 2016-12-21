extern crate graph;
extern crate prim;

use graph::Graph;

#[test]
fn basic(){
	let mut g = Graph::new();
	let a = g.add_vertex("a");
	let b = g.add_vertex("b");
	let c = g.add_vertex("c");

	g.add_dir_edge(5, a, b);
	g.add_edge(3, b, c);
	g.add_edge(4, c, a);

	assert!(prim::mst(g) == vec![1, 2]);
}