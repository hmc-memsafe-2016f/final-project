# final-project
A repository to submit their final project PRs against

# Review of `petgraph`
Petgraph offers 3 types of graphs. Graph it says is an adjacency list, StableGraph
it says is similar but keeps indices stable across removals, and GraphMap which uses
an adjacency list plus hash table.
It implements these graph algorithms:
 * bellman_ford
 * condensation
 * connected_components
 * dijkstra
 * has_path_connecting
 * is_cyclic_directed
 * is_cyclic_undirected
 * is_isomorphic
 * is_isomorphic_matching
 * kosaraju_scc
 * min_spanning_tree
 * scc
 * tarjan_scc
 * toposort
The API seems fairly safe although it will throw errors if you try to do bad things
such as run an algorithm on a graph that it can't be run on. It will panic on certain
functions if capacity overflows usize. It can also panic if you try to add nodes when
the graph is already at the maximum number of nodes or if you try to do things with
nodes that don't exist (like add an edge to a node that doesn't exist).

A strong point of the API is what you can do with it. Graphs can be directed or
undirected. There are a number of graph algorithms implemented. A weak point is
that it says it is adjacency list representation and I don't see a way to get an
adjacency matrix.

Since it says that it uses adjacency lists I suspect it has some type of linked
list type structure under the hood. It also makes the user specify the maximimum
size of a graph, so I suspect it might you something like an array of a fixed size
to store the elements.