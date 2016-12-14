
//Ross Mawhorter, Luis Viorney

type NodeReference<N, E> = Rc<RefCell<Node<N, E>>>;

pub type WeakNodeReference<N, E> = Weak<RefCell<Node<N, E>>>;

pub struct Graph<N, E: Ord>
{
	vertices:	Vec<NodeReference<N, E>>
}

struct Edge<E>
{
	node:	NodeReference,
	weight:	E
}

struct Node<N, E>
{
	data:	N,
	edges:	Vec<Edge<E>>
}

impl<N, E> Graph<N, E>
{
///Creates a new `Graph`
	pub fn new() -> Self
	{
		vertices = Vec::new();
	}

	///Gets an iterator over the nodes
	pub fn nodes(&self) -> impl Iterator<Item=WeakNodeReference<N, E>>
	{
		vertices.iter().map(|x| x.downgrade())
	}

	//mutable iterator over the nodes??

	///Gets an iterator of the neighbors of a node
	///`Ok` if the node exists
	///`Err` if it doesn't exist anymore
	pub fn neighbors(&self, node: &WeakNodeReference) -> Result<impl Iterator<Item=WeakNodeReference<N, E>>, ()>
	{
		match node.upgrade()
		{
			Some(strong_node)	=> Ok(strong_node.edges.iter().map(|x| x.downgrade())),
			None 				=> Err()
		}
	}

	///Creates an edge from one node to another (directed)
	///Updates the edge weight if an edge already existed
	///Ensures no || edges
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if the node doesn't exist anymore
	pub fn create_edge(&mut self, from: &WeakNodeReference, to: &WeakNodeReference, weight: E) -> Result<(), ()>
	{

		/*
		from.upgrade().map(|strong_from| 
			to.upgrade().map(|strong_to| 
				let edge{node: strong_to, weight: weight};
				let from_ref = strong_from.borrow_mut();
				*from_ref.edges.push_front(edge);
				)).ok_or(())
		*/

		match from.upgrade()
		{
			Some(strong_from)	=> match to.upgrade()
								   {
								   		Some(strong_to) => {
								   							//Create an edge
								   							let edge = Edge::<E>{node: strong_to, weight: weight};

								   							let from_ref = strong_from.borrow_mut();

								   							//Filter out all edges already existing to to
								   							*from_ref.edges.retain(|x| *x.node != *strong_to);

								   							//add in the new edge
								   							*from_ref.edges.push_front(edge);
								   							Ok()
								   							},
								   		None			=> Err()
								   },
			None				=> Err()
		}
	}

	///Creates an undirected edge, represented as two edges with the same weight
	///While it's possible to delete this edge with delete_edge, it should be deleted
	///Updates the weight if an edge already existed
	///Ensures no || edges
	///with `delete_edge_undirected`
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if either node doesn't exist anymore
	pub fn create_edge_undirected(&mut self, from: &WeakNodeReference, to: &WeakNodeReference, weight: E) -> Result<(), ()>
	{
		/*
		from.upgrade().map(|strong_from| 
			to.upgrade().map(|strong_to| 
				let to_edge = Edge<E>{node: strong_to, weight: weight};
				let from_edge = Edge<E>{node: strong_from, weight:weight};

				let from_ref = strong_from.borrow_mut();
				let to_ref = strong_to.borrow_mut();

				*from_ref.edges.push_front(to_edge);
				*to_ref.edges.push_front(from_edge);
				)).ok_or(())
		*/

		match from.upgrade()
		{
			Some(strong_from)	=> match to.upgrade()
								   {
								   		Some(strong_to) => {
								   							let to_edge = Edge::<E>{node: strong_to, weight: weight};
								   							let from_edge = Edge::<E>{node: strong_from, weight: weight};
								   							let from_ref = strong_from.borrow_mut();
								   							let to_ref = strong_to.borrow_mut();

															//Filter out all edges already existing to to
								   							*from_ref.edges.retain(|x| *x.node != *strong_to);
								   							*to_ref.edges.retain(|x| *x.node != *strong_from);

								   							*from_ref.edges.push_front(to_edge);
								   							*to_ref.edges.push_front(from_edge);
								   							Ok()
								   							},
								   		None			=> Err()
								   },
			None				=> Err()
		}
	}

	///Creates a node
	pub fn create_node(&mut self, data: N) -> WeakNodeReference
	{
		//node with an empty edge list
		node = Node::<N, E>{data: data, edges: vec!()};

		//wrap it in an Rc / RefCell
		let node_ref = Rc::new(RefCell::new(node));

		//store that reference in the graph
		vertices.push_front(node_ref);

		//give the user back a weak to it
		node_ref.downgrade()
	}

	///Deletes a directed edge that points from the specified node to another
	///`Ok` if the edge was deleted normally or if the edge didn't exist
	///`Err` if either node doesn't exist anymore
	pub fn delete_edge(&mut self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<(), ()>
	{

		match to.upgrade()
		{
			Ok(strong_to) 	=> match from.upgrade()
							{
								Ok(strong_from)	=> {
													strong_from.borrow_mut().edges.retain(|x| *x.node != *strong_to);
													Ok()
													},
								None			=> Err()
							},
			Err				=> Err()
		}
	}

	///Deletes an undirected edge that points between the two nodes
	///Won't delete edges between nodes if their weights are different
	///because it assumes that they are two unrelated undirected edges
	///`Ok` if the edges were deleted normally or if the edge didn't exist
	///`Err` if their weights were different, or if either node doesn't exist anymore
	pub fn delete_edge_undirected(&mut self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<(), ()>
	{
		match to.upgrade()
		{
			Ok(strong_to) 	=> match from.upgrade()
							{
								Ok(strong_from)	=> {
													let to_ref = strong_to.borrow_mut();
													let from_ref = strong_to.borrow_mut();

													//find the edge from to to from
													match to_ref.edges.into_iter().find(|x| *x.node == *strong_from)
													{
														Some(edge1) =>	match from_ref.edges.into_iter().find(|x| *x.node == *strong_to && x.weight == edge1.weight)
																		{
																			Some(edge2)	=> {
																							to_ref.edges.retain(|x| *x.node != *strong_from);
																							from_ref.edges.retain(|x| *x.node != *strong_to);
																							Ok()
																							},
																			None		=> Err()
																		},
														None		=>	Err()
													}
													},
								None			=> Err()
							},
			Err				=> Err()
		}
	}

	///Deletes a node
	///`Ok` if the node was deleted normally
	///`Err` if the node doesn't exist anymore
	pub fn delete_node(&mut self, node: WeakNodeReference) -> Result<(), ()>
	{
		//get iterator over the neighbors
		match self.neighbors(node)
		{
			Ok(iter)	=> {
							//delete all edges from each neighbor
							iter.map(|x| self.delete_edge(&x, &node));
							//delete the node
							match node.upgrade()
							{
								Some(strong_node)	=> {
														self.nodes.retain(|x| *x == *node);
														Ok()
														},
								None				=> Err()
							}
							},
			Err			=> Err()
		}
	}

	///Checks if a node still exists
	///`true` if the node is still in the graph
	///`false` if it has been deleted
	pub fn check_node(&self, node: &WeakNodeReference) -> bool
	{
		match node.upgrade()
		{
			Some(strong_node) => true,
			None => false
		}
	}

	///Immutably borrows the data of a node
	pub fn get_node(&self, node: &WeakNodeReference) -> Result<&N, ()>
	{
		match node.upgrade()
		{
			Some(strong_node) => Ok(strong_node.borrow()),
			None => Err()
		}
	}

	///Mutably borrows the data of a node
	pub fn get_node_mut(&self, node: &WeakNodeReference) -> Result<&mut N, ()>
	{
		match node.upgrade()
		{
			Some(strong_node) => Ok(strong_node.borrow_mut()),
			None => Err()
		}
	}

	//Immutably borrows the weight of an edge
	pub fn get_edge(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<&E, ()>
	{
		match from.upgrade()
		{
			Some(strong_from)	=> match to.upgrade()
									{
										Some(strong_to)	=> match strong_from.borrow_mut().edges.into_iter().find(|x| *x.node == *strong_to)
															{
																Some(edge)	=> Ok(&edge.weight),
																None 		=> Err()
															},
										None 			=> Err()
									},
			None 				=> Err()
		}
	}

	///Mutably borrows the weight of an edge
	pub fn get_edge_mut(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<&mut E, ()>
	{
		match from.upgrade()
		{
			Some(strong_from)	=> match to.upgrade()
									{
										Some(strong_to)	=> match strong_from.borrow_mut().edges.into_iter().find(|x| *x.node == *strong_to)
															{
																Some(edge)	=> Ok(&mut edge.weight),
																None 		=> Err()
															},
										None 			=> Err()
									},
			None 				=> Err()
		}
	}

	///Dijkstra's shortest-path algorithm
	///Returns a list of node indices. Traversing along edges to these
	///nodes in order from the starting edge is the shortest path from
	///the start node to the end node
	pub fn dijkstras(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Vec<WeakNodeReference>
	{
		unimplemented!()
	}
}
