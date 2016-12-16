
//Ross Mawhorter, Luis Viornery

#![feature(conservative_impl_trait)]
#![feature(ptr_eq)]
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::mem;
use std::ops::Add;

type NodeReference<N, E> = Rc<RefCell<Node<N, E>>>;

pub struct PossibleNode<N, E>(Weak<RefCell<Node<N, E>>>);

pub struct Graph<N, E: Eq+Clone>
{
	vertices:	Vec<NodeReference<N, E>>
}

struct Edge<N, E>
{
	node:	NodeReference<N, E>,
	weight:	E
}

struct Node<N, E>
{
	data:	N,
	edges:	Vec<Edge<N, E>>
}

impl<N, E: Eq+Clone> Graph<N, E>
{
///Creates a new `Graph`
	pub fn new() -> Self
	{
		Graph{
            vertices: Vec::new(),
        }
	}

	///Gets an iterator over the nodes
	pub fn nodes(&self) -> Vec<PossibleNode<N, E>>
	{
		self.vertices.iter().map(|x| PossibleNode(Rc::downgrade(x))).collect()
	}

	//mutable iterator over the nodes??

	///Gets an iterator of the neighbors of a node
	///`Ok` if the node exists
	///`Err` if it doesn't exist anymore
	pub fn neighbors(&self, node: &PossibleNode<N, E>) -> Result<Vec<PossibleNode<N, E>>,()>
	{
		match node.0.upgrade()
		{
			Some(strong_node)	=> Ok((*strong_node).borrow().edges.iter().map(|x| PossibleNode(Rc::downgrade(&x.node))).collect()),
			None 				=> Err(())
		}
	}

	///Creates an edge from one node to another (directed)
	///Updates the edge weight if an edge already existed
	///Ensures no || edges
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if the node doesn't exist anymore
	pub fn create_edge(&mut self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>, weight: E) -> Result<(), ()>
	{
		match from.0.upgrade().and_then(|strong_from|
			to.0.upgrade().and_then(|strong_to|
			{
				//Create an edge
				let edge = Edge::<N, E>{node: strong_to.clone(), weight: weight.clone()};
				let mut from_ref = strong_from.borrow_mut();

				//filter out all eddges already existing to to
				from_ref.edges.retain(|x| Rc::ptr_eq(&x.node, &strong_to));

				//add in the new edge
				from_ref.edges.push(edge);
				Some(())
			}
			))
		{
			Some(_) => Ok(()),
			None	=> Err(())
		}
	}

	///Creates an undirected edge, represented as two edges with the same weight
	///While it's possible to delete this edge with delete_edge, it should be deleted
	///Updates the weight if an edge already existed
	///Ensures no || edges
	///with `delete_edge_undirected`
	///`Ok` if the edge was created normally
	///`Err` if the edge already existed or if either node doesn't exist anymore
	pub fn create_edge_undirected(&mut self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>, weight: E) -> Result<(), ()>
	{
		match from.0.upgrade().and_then(|strong_from|
			to.0.upgrade().and_then(|strong_to|
			{
				let to_edge = Edge::<N, E>{node: strong_to.clone(), weight: weight.clone()};
				let from_edge = Edge::<N, E>{node: strong_from.clone(), weight: weight.clone()};
				let mut from_ref = strong_from.borrow_mut();
				let mut to_ref = strong_to.borrow_mut();

				//Filter out all edges already existing to to
				from_ref.edges.retain(|x| !Rc::ptr_eq(&x.node,&strong_to));
				to_ref.edges.retain(|x| !Rc::ptr_eq(&x.node,&strong_from));

				from_ref.edges.push(to_edge);
				to_ref.edges.push(from_edge);
				Some(())
			}
			))
		{
			Some(_) => Ok(()),
			None	=> Err(())
		}
	}

	///Creates a node
	pub fn create_node(&mut self, data: N) -> PossibleNode<N, E>
	{
		//node with an empty edge list
		let node = Node::<N, E>{data: data, edges: vec!()};

		//wrap it in an Rc / RefCell
		let node_ref = Rc::new(RefCell::new(node));

		//store that reference in the graph
		self.vertices.push(node_ref.clone());

		//give the user back a weak to it
		PossibleNode(Rc::downgrade(&node_ref))
	}

	///Deletes a directed edge that points from the specified node to another
	///`Ok` if the edge was deleted normally or if the edge didn't exist
	///`Err` if either node doesn't exist anymore
	pub fn delete_edge(&mut self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>) -> Result<(), ()>
	{
		match from.0.upgrade().and_then(|strong_from|
			to.0.upgrade().and_then(|strong_to|
			{
				strong_from.borrow_mut().edges.retain(|x| !Rc::ptr_eq(&x.node, &strong_to));
				Some(())
			}
			))
		{
			Some(_) => Ok(()),
			None	=> Err(())
		}
	}

	///Deletes an undirected edge that points between the two nodes
	///Won't delete edges between nodes if their weights are different
	///because it assumes that they are two unrelated undirected edges
	///`Ok` if the edges were deleted normally or if the edge didn't exist
	///`Err` if their weights were different, or if either node doesn't exist anymore
	pub fn delete_edge_undirected(&mut self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>) -> Result<(), ()>
	{
		let strong_from = try!(from.0.upgrade().ok_or(()));
        let strong_to = try!(to.0.upgrade().ok_or(()));
        let mut to_ref = strong_to.borrow_mut();
        let mut from_ref = strong_from.borrow_mut();

        let to_edge_weight = try!(to_ref.edges.iter()
            .find(|x| Rc::ptr_eq(&x.node,&strong_from))
            .and_then(|to_edge| Some(to_edge.weight.clone())).ok_or(()));
        let from_edge_weight = try!(from_ref.edges.iter()
            .find(|x| Rc::ptr_eq(&x.node,&strong_to))
            .and_then(|from_edge| Some(from_edge.weight.clone())).ok_or(()));
        if to_edge_weight == from_edge_weight
        {
            to_ref.edges.retain(|x| !Rc::ptr_eq(&x.node, &strong_from));
            from_ref.edges.retain(|x| !Rc::ptr_eq(&x.node, &strong_to));
            Ok(())
        }
        else 
        {
            Err(())
        }
	}

	///Deletes a node
	///`Ok` if the node was deleted normally
	///`Err` if the node doesn't exist anymore
	pub fn delete_node(&mut self, node: PossibleNode <N, E>) -> Result<(), ()>
	{
		//get iterator over the neighbors
		match self.neighbors(&node)
		{
			Ok(collection)	=> {
							//delete all edges from each neighbor
							collection.into_iter().map(|x| self.delete_edge(&x, &node)).count();
							//delete the node
							match node.0.upgrade()
								.and_then(|strong_node|
                                    {
                                        self.vertices.retain(|x| !Rc::ptr_eq(&x, &strong_node));
                                        Some(())
                                    }
                                )
							{
								Some(_)	=>	Ok(()),
								None	=> Err(())
							}
							},
			Err(())		=> Err(())
		}
	}

	///Checks if a node still exists
	///`true` if the node is still in the graph
	///`false` if it has been deleted
	pub fn check_node(&self, node: &PossibleNode<N, E>) -> bool
	{
		match node.0.upgrade()
		{
			Some(_) => true,
			None => false
		}
	}

	///Maps a function on the data of a node. Not allowed to change the data.
	pub fn map_node<F, R>(&self, node: &PossibleNode<N, E>, func: F) -> Result<R,()>
        where F: FnOnce(&N) -> R
	{
		match node.0.upgrade()
		{
			Some(strong_node) => Ok(func(&(*strong_node).borrow().data)),
			None => Err(())
		}
	}

	///Maps a function on the data of a node. Allowed to change the data.
	pub fn map_node_mut<F, R>(&self, node: &PossibleNode<N, E>, func: F) -> Result<R,()>
        where F: FnOnce(&mut N) -> R
	{
		match node.0.upgrade()
		{
			Some(strong_node) => Ok(func(&mut (*strong_node).borrow_mut().data)),
			None => Err(())
		}
	}

	///Replaces the data of a node with new data and returns the old data
	pub fn replace_node(&self, node: &PossibleNode<N, E>, node_data: N) -> Result<N,()>
	{
		match node.0.upgrade()
		{
			Some(strong_node) => Ok(mem::replace(&mut (*strong_node).borrow_mut().data, node_data)),
			None => Err(())
		}
	}

	//Returns the weight of an edge
	pub fn get_edge(&self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>) -> Result<E, ()>
	{
		match from.0.upgrade().and_then(|strong_from|
			to.0.upgrade().and_then(|strong_to|
			{
				//the first edge in from's edge list that points to to
				match strong_from.borrow_mut().edges.iter().find(|x| Rc::ptr_eq(&x.node, &strong_to))
				{
					Some(edge)	=> Some(edge.weight.clone()),
					None 		=> None
				}
			}
			))
		{
			Some(edge_weight) => Ok(edge_weight),
			None	=> Err(())
		}
	}

	//Maps a function onto the weight of an edge. Allowed to change the weight.
	pub fn map_edge_mut<F, R>(&self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>, func: F) -> Result<R,()>
        where F: FnOnce(&mut E) -> R
	{
		let strong_from = try!(from.0.upgrade().ok_or(()));
		let strong_to = try!(to.0.upgrade().ok_or(()));
        
        let mut borrowed_from = strong_from.borrow_mut();
        
		borrowed_from.edges.iter_mut().find(|x| Rc::ptr_eq(&x.node, &strong_to)).map(|x| func(&mut x.weight)).ok_or(())
	}
}

#[derive(Clone)]
struct DijkstraEntry<N, E>
{
    this_node: NodeReference<N,E>,
    previous_node: Option<NodeReference<N,E>>,
    distance: Option<E>,
}

impl<N, E: Eq+Clone+Add> Graph<N, E>
{
	///Dijkstra's shortest-path algorithm
	///Returns a list of node indices. Traversing along edges to these
	///nodes in order from the starting edge is the shortest path from
	///the start node to the end node
	pub fn dijkstras(&self, from: &PossibleNode<N, E>, to: &PossibleNode<N, E>) -> Result<Vec<PossibleNode<N, E>>,()>
	{
        let strong_from = try!(from.0.upgrade().ok_or(()));
		let strong_to = try!(to.0.upgrade().ok_or(()));

        let node_vec = Vec::new();
        let searched_nodes = Vec::new();
        for node in self.vertices
        {
            node_vec.push(DijkstraEntry
            {
                this_node: node.clone(),
                previous_node: None,
                distance: None,
            });
        }

        let current_node = node_vec.iter().find(|x| Rc::ptr_eq(&x.this_node, &strong_from).unwrap();
        
        while !Rc::ptr_eq(&current_node.this_node, &strong_to)
        {
            for weak_node in self.neighbors(current_node)
            {
                let node = weak_node.0.upgrade().unwrap();
                for entry in node_vec
                {
                    if Rc::ptr_eq(&entry.this_node, &node)
                    {
                        let alt = current_node.edges.iter().find(|x| Rc::ptr_eq(&x.node, &node)).unwrap().weight;
                        if current_node.distance.is_some()
                        {
                            alt = alt + current_node.distance.unwrap();
                        }
                        match entry.distance
                        {
                            None                => {
                                                    	entry.distance = Some(alt);
                                                    	entry.previous_node = current_node.this_node.clone();
                                                    }
                            Some(current_dist)  => if current_dist > alt
                                                    {
                                                        current_dist = Some(alt);
                                                        entry.previous_node = current_node.this_node.clone();
                                                    }
                        }
                	}
                }
            }

            let min_value = None;
            let min_value_index = None;
            for (entry in 0..node_vec.len())
            {
            	if node_vec.get(entry).distance.is_some()
            	{
            		if min_value.is_some()
            		{
            			if min_value > node_vec.get(entry).distance
            			{
            				min_value = node_vec.get(entry).distance;
            				min_value_index = entry;
            			}
            		}
            		else
            		{
            			min_value = node_vec.get(entry).distance;
            			min_value_index = entry;
            		}
            	}
            }

            //there's no way forward
            if min_value.is_none()
            {
            	return Err(());
            }

            current_node = node_vec.get(min_value_index).clone();

            searched_nodes.push(current_node.clone());

            node_vec.remove(min_value_index);
        }

        let out = Vec::new();

        //now we unwrap backwards finding the previous node from the destination
        while !Rc::ptr_eq(&current_node.this_node, &strong_from)
        {
        	out.push(current_node.this_node.clone().downgrade());

        	current_node = searched_nodes.find(|x| Rc::ptr_eq(&current_node.previous_node, &x.this_node)).unwrap()
        }
	}
}
