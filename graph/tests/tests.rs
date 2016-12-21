extern crate graph;


use graph::Graph;


#[test]
fn basic(){
	let mut g = Graph::new();
	let a = g.add_vertex("Alice");
	let b = g.add_vertex("Bob");
	let c = g.add_vertex("Carol");

	g.add_dir_edge("tutors", a, b);
	g.add_edge("studies with", b, c);
	g.add_edge("does sportsball with", c, a);

	assert!(*g.get_vertices() == vec!["Alice", "Bob", "Carol"]);
	assert!(*g.get_vertex(1) == "Bob");
	assert!(g.edge_data == vec!["tutors", "studies with", "does sportsball with"]);
	assert!(g.get_edge(c, b) == Some(&"studies with"));
	assert!(g.get_neighbors(b) == vec![&c]);
}

	
