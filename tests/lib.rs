extern crate mygraph;

use mygraph::{Graph};

#[derive(Clone, Debug)]
struct Node { data: char }

#[test]
fn graph() {
	let mut g = Graph::new(true);
	assert_eq!(0, g.num_nodes());
	assert_eq!(0, g.num_edges());

	let v1 = g.add_node(Node{ data: 'a' });
	assert_eq!(g.get_node_data(0).data, 'a');
	let v2 = g.add_node(Node{ data: 'b' });
	assert_eq!(g.get_node_data(1).data, 'b');
	let v3 = g.add_node(Node{ data: 'c' });
	assert_eq!(g.get_node_data(2).data, 'c');
	assert_eq!(3, g.num_nodes());

	g.add_edge(v1, v2, 12 );
	assert_eq!(g.get_edge_weight(0, 1), 12);
	assert_eq!(g.has_edge(0, 1) , true);
	assert_eq!(1, g.num_edges());

	g.add_edge(v2, v3, 8 );
	assert_eq!(g.get_edge_weight(1, 2), 8);
	assert_eq!(g.has_edge(1, 2) , true);
	assert_eq!(2, g.num_edges());

	assert_eq!(g.has_edge(2, 0) , false);
	assert_eq!(g.has_edge(1, 0) , false);

	assert_eq!(g.get_neighbors(0).count(), 1);
	assert_eq!(g.get_neighbors(1).count(), 1);
	assert_eq!(g.get_neighbors(2).count(), 0);

	assert_eq!(g.get_nodes().count(), 3);
}