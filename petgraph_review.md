# [`petgraph::graph::Graph`](https://docs.rs/petgraph/0.4.1/petgraph/graph/struct.Graph.html) API Review

The `petgraph` API provides a graph data-structure where you can use arbitrary values for nodes and edges,
where the arbitrary edge value is equivalent to that edge's weight. It can also be either directed or undirected.

A simple set of features includes the ability to construct graphs of arbitrary size,
add, remove, and mutate both edges and nodes, and utility functions to provide statistics
of the graph (such as number of nodes) as well as simple information regarding the nieghbors
of a given node.

The graph is mostly safe, however a panic can occur if:
- Any type of overflow might occur. The graph's size is constrained by an index type,
  and certain operations may cause that to overflow (e.g. adding too many nodes or edges,
  or reserving memory, etc).
- Any operation to add a new node/edge or mutate an existing one will panic if the
  given node/edge doesn't actually exist.
- Additionally, various functions return iterators that may be empty; if a user
  attempts to use one of these empty iterators that will cause an error.
- Likewise, several functions return `Option<T>`s, and attempting to use them if
  they are `None` will cause a panic to occur.

The API seems to be pretty solid in general, however there are a few points that are not ideal.
- Removing a node invalidates the last node index, but not any of the others. While it is
  nice that you can avoid invalidation of all the indices, in general it is more common
  that all iterators (or indices) are invalidated by operations like remove.
- Similarly, the fact that the implementation is so easily exposed by providing a `NodeIndex<N>`
  is a bit of a code-smell; it hampers future changes in implementation where the graph
  doesn't use arrays.
- Additionally, the fact that an empty iterator can be returned is slightly annoying; it seems
  like using `Option<T>` everywhere would create a more consistant interface.

`petgraph` implements its graphs as adjacency-lists. This doesn't require any particular
investigative work or looking at the source, the documentation states it at the very beginning.
