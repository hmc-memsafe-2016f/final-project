## Petgraph API Review

### High-Level Features
Petgraph gives you an API to create graphs and do things with them. Petgraph
provides three structs, `Graph`, `StableGraph`, and `GraphMap`. `Graph` is a
normal graph representation, and `StableGraph` is like `Graph` but the indices
of nodes and edges are kept constant. This is in contrast to `Graph`, where the
removal of edges and nodes could change the indices. `GraphMap` is also a graph
datastructure, but uses an associative array, where `Graph` and `StableGraph`
both use adjacency lists.

Looking at the methods, `Graph` and other structs have many, many functions
associated with it. The different graphs let you do anything from add a node, to
returning an iterator over nodes that do not have incoming or outgoing edges
(`externals`), to reversing the direction of all edges (`reverse`).

In the graph data structures, there is a big emphasis on determining information
about a node's neighbors and edges leaving and going into a node, etc. All this
information is useful when implementing graph algorithms.

The API also provides graph algorithms that can be used on the graphs created,
including Djikstra's, Bellman-Ford, etc.

###Saftey and Possibility of Panic

There are a couple of spots where the user could cause a panic, and they seem to
have all been described in the API. For example:

-`add_node` could panic if the `Graph` is already has a maximum number of nodes
    for its index type
-A Panic could be caused when adding an `edge` if the nodes needed do not exist,
    or there are too many edges for the index type used (has maximum number of
    edges).
-Panics can happen if something overflows a type. For example, in
    `reserve_`{nodes, edges}, if the additional size added overflows `usize`,
    then a panic can happen.

To sum it up, panics in `petgraph` general happen if something overflows, or
something requested/needed (usually edges or nodes) do not exist.

###Strong and Weak Points of the API

The strong points of the API seem to be that it is very extensive. Looking
through the methods, there seems to be everything you'd possibly want for at
least the common algorithms.

What I don't like is `Graph` not keeping indices constant, and requiring a
whole separate data structure in order to promise constsant indices, regardless
of removals. We aren't allowed to look at the internals so I can only speculate
that maybe `Graph` was the initial implementation, and there was not yet a
downside to changing indices when deleting. There could be other reasons, of
course, but we are not privy to them at this time.

###Guess at how Petgraph Represents Graph (Under the Hood)
The documentation states that for `Graph` and `StableGraph`, the data structures
use adjacency lists. And for `GraphMap`, the data structure uses associative
arrays (aka maps) to implement its graphs.

