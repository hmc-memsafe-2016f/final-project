
# A Review of the Petgraph API

1. Describe the high-level features the API provides.

There are three graph representations, `Graph`, `Stable Graph`, and `GraphMap`.

Graph and Stable Graph are adjacency list representations of graphs; thus
requiring:
 * `O(|V| + |E|)` space to store
 * `O(|V| + |E|)` time to iterate over all nodes and edges
 * `O( maximum degree in graph )` time to perform an edge lookup

Both allow nodes and edges to hold arbitrary data and the petgraph API
provides Indices, which are like iterators.  Stable Graph essentially
prevents invalidations of Indices even when deleting nodes.

Neither graph's Indices are invalidated through inserts.

Petgraph also provides an option for constant time edge lookup, a `GraphMap`.

2. Analyze how safe using the API is (where could a user cause a panic?)

Indices are not very safe because you can invalidate your indices.  Rust can't
know.

3. Describe strong and weak points of the API.

It is a full library, with many member functions and support for algorithm
modules, visualizaiton, and data structures.

There is a lot of support for the common graph operations used in algorithms,
such as visiting nodes, DFS, BFS, and cycle checking.

4. Provide a high-level guess at how petgraph might represent graphs under the
hood.


