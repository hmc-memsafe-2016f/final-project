

type NodeReference<N, E> = Rc<RefCell<Node<N, E>>>;

pub type WeakNodeReference<N, E> = Weak<RefCell<Node<N, E>>>;

pub struct Graph<N, E: Ord>
{
	vertices:	Vec<NodeReference<N, E>>
}

struct Edge<E>
{
	node:	NodeReference
	weight:	E
}

struct Node<N, E>
{
	data:	N
	edges:	Vec<Edge<E>>
}

impl<N, E> Graph<N, E>
{
///Creates a new `Graph`
	pub fn new() -> Self
	{

	}

	///Gets an iterator over the nodes
	pub fn nodes(&self) -> iterator<WeakNodeReference>
	{

	}

	//mutable iterator over the nodes??

	///Gets an iterator of the neighbors of a node
	///`Ok` if the node exists
	///`Err` if it doesn't exist anymore
	pub fn neighbors(&self, node: &WeakNodeReference) -> Result<iterator<WeakNodeReference>, ()>
	{
		match node.upgrade()
		{
			Some(strong_node) => Ok(...)
			None() => Err()
		}
	}

	///Creates an edge from one node to another (directed)
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if the node doesn't exist anymore
	pub fn create_edge(&mut self, from: &WeakNodeReference, to: &WeakNodeReference, weight: E) -> Result<(), ()>
	{

	}

	///Creates an undirected edge, represented as two edges with the same weight
	///While it's possible to delete this edge with delete_edge, it should be deleted
	///with `delete_edge_undirected`
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if either node doesn't exist anymore
	pub fn create_edge_undirected(&mut self, from: &WeakNodeReference, to: &WeakNodeReference, weight: E) -> Result<(), ()>
	{

	}

	///Creates a node
	pub fn create_node(&mut self, data: N) -> WeakNodeReference
	{
		//node with an empty edge list
		node = Node<N, E>{data, vec!()};

		//wrap it in an Rc / RefCell
		NodeReference node_ref = Rc::new(RefCel::new(node));

		//store that reference in the graph
		vertices.push_front(node_ref);

		//give the user back a weak to it
		node_ref.downgrade()
	}

	///Deletes a directed edge that points from the specified node to another
	///`Ok` if the edge was deleted normally
	///`Err` if the edge didn't exist or if either node doesn't exist anymore
	pub fn delete_edge(&mut self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<(), ()>
	{

	}

	///Deletes an undirected edge that points between the two nodes
	///Won't delete edges between nodes if their weights are different
	///because it assumes that they are two unrelated undirected edges
	///`Ok` if the edges were deleted normally
	///`Err` if the edges didn't exist, or if their weights were different, or if either node doesn't exist anymore
	pub fn delete_edge_undirected(&mut self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<(), ()>
	{

	}

	///Deletes a node
	///`Ok` if the node was deleted normally
	///`Err` if the node doesn't exist anymore
	pub fn delete_node(&mut self, node: WeakNodeReference) -> Result<(), ()>
	{
		//get iterator over the neighbors
		//delete all edges from each neighbor
		//delete the node
	}

	///Checks if a node still exists
	///`true` if the node is still in the graph
	///`false` if it has been deleted
	pub fn check_node(&self, node: &WeakNodeReference) -> bool
	{
		match node.upgrade()
		{
			Some(strong_node) => true
			None() => false
		}
	}

	///Immutably borrows the data of a node
	pub fn get_node(&self, node: &WeakNodeReference) -> Result<&N, ()>
	{
		match node.upgrade()
		{
			Some(strong_node) => Ok(strong_node.borrow())
			None() => Err()
		}
	}

	///Mutably borrows the data of a node
	pub fn get_node_mut(&self, node: &WeakNodeReference) -> Result<&mut N, ()>
	{
		match node.upgrade()
		{
			Some(strong_node) => Ok(strong_node.borrow_mut())
			None() => Err()
		}
	}

	//Immutably borrows the weight of an edge
	pub fn get_edge(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<&E, ()>
	{

	}

	///Mutably borrows the weight of an edge
	pub fn get_edge_mut(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Result<&mut E, ()>
	{

	}

	///Dijkstra's shortest-path algorithm
	///Returns a list of node indices. Traversing along edges to these
	///nodes in order from the starting edge is the shortest path from
	///the start node to the end node
	pub fn dijkstras(&self, from: &WeakNodeReference, to: &WeakNodeReference) -> Vec<WeakNodeReference>
	{

	}
}

/*
//Ross Mawhorter, Luis Viorney

///
///This is a graph which represents each vertex as an adjacency list of edges.
///Each edge has a weight, which should implement Ord as a way to do 
///
///You get node indices as usizes that you can use to traverse
///and mutate the graph.
///
#[derive(Debug)]
pub struct Graph<N, E: Ord>
{
	vertices:	Vec<Node<N, E>>
}

#[derive(Debug)]
struct Node<N, E>
{
	data:		N,
	edgeList:	Vec<Edge<E>>
}

#[derive(Debug)]
struct Edge<E>
{
	index:	usize,
	weight:	E
}

impl<N, E> Graph<N, E>
{
	///Creates a new `Graph`
	pub fn new() -> Self
	{

	}

	///Gets an iterator over the nodes
	pub fn nodes(&self) -> iterator<usize>
	{

	}

	//mutable iterator over the nodes??

	///Gets an iterator of the neighbors of a node
	pub fn neighbors(&self, nodeIndex: usize) -> iterator<usize>
	{

	}

	///Creates an edge from one node to another (directed)
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed
	pub fn create_edge(&mut self, from: usize, to: usize, weight: E) -> Result<()>
	{

	}

	///Creates an undirected edge, represented as two edges with the same weight
	///While it's possible to delete this edge with delete_edge, it should be deleted
	///with `delete_edge_undirected`
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed
	pub fn create_edge_undirected(&mut self, from: usize, to: usize, weight: E) -> Result<()>
	{

	}

	///Creates a node
	pub fn create_node(&mut self, data: N) -> usize
	{

	}

	///Deletes a directed edge that points from the specified node to another
	///`Ok` if the edge was deleted normally
	///`Err` if the edge didn't exist
	pub fn delete_edge(&mut self, from: usize, to: usize) -> Result<()>
	{

	}

	///Deletes an undirected edge that points between the two nodes
	///Won't delete edges between nodes if their weights are different
	///because it assumes that they are two unrelated undirected edges
	///`Ok` if the edges were deleted normally
	///`Err` if the edges didn't exist, or if their weights were different
	pub fn delete_edge_undirected(&mut self, from: usize, to: usize) -> Result<()>
	{

	}

	///Deletes a node
	///Preferably doesn't mess up existing nodeIndex usizes
	pub fn delete_node(&mut self, nodeIndex: usize)
	{

	}

	///Immutably borrows the data of a node
	pub fn get_node(&self, nodeIndex: usize) -> &N
	{

	}

	///Mutably borrows the data of a node
	pub fn get_node_mut(&self, nodeIndex: usize) -> &mut N
	{

	}

	//Immutably borrows the weight of an edge
	pub fn get_edge(&self, from: usize, to: usize) -> &E
	{

	}

	///Mutably borrows the weight of an edge
	pub fn get_edge_mut(&self, from: usize, to: usize) -> &mut E
	{

	}

	///Dijkstra's shortest-path algorithm
	///Returns a list of node indices. Traversing along edges to these
	///nodes in order from the starting edge is the shortest path from
	///the start node to the end node
	pub fn dijkstras(&self, from: usize, to: usize) -> Vec<usize>
	{

	}
}
*/