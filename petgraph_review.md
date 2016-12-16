
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

Petgraph also provides an option with constant time edge lookup, a `GraphMap`.
This is an adjacency list backed by a hash table. The hash table stores
node weights, which can be of arbitrary type as long as they implement the
`Hash`, `Eq`, and `Ord` traits. `GraphMap` also supports edges weighted by
arbitrary types.

2. Analyze how safe using the API is (where could a user cause a panic?)

There are several ways to cause a panic using only functions provided by
the API. In particular, there are multiple ways to cause a panic even when
using a `StableGraph`. For instance, adding edges or vertices past the
maximum number allowed, and looking up nodes, edges, or indices that don't
exist all cause a panic for both `Graph` and `StableGraph`. `GraphMap` does
not claim to panic for any function provided by the API.

3. Describe strong and weak points of the API.

One strength of petgraph is its wealth of functionality. The full library provides
many member functions and support for algorithm modules, visualization, and data
structures. There is a lot of support for the common graph operations used in
algorithms, such as visiting nodes, DFS, BFS, and cycle checking. Additionally,
petgraph strikes a good balance by providing data structures that each prioritize
safety, speed, and memory usage differently.

On the other hand, the paradigm in which every template argument represents a weight
seems unintuitive and we will consider avoiding this in making our API. Petgraph is
also very general, which is ultimately more useful, but comes at the expense of
immediacy for newcomers or users who just want a functioning graph API and don't
care strongly about performance.

4. Provide a high-level guess at how petgraph might represent graphs under the
hood.

`Graph` and `StableGraph` both explicitly claim to be adjacency lists, so they are
probably represented in memory as a sequence of linked lists, holding the edges.

`GraphMap` is more exotic, but we suspect it's something like an adjacency list,
whose edges are the values in a hashmap keyed by pairs of vertices. The reason we
suspect these are the keys is that the API requires that keys be ordered "so that
the implementation can order the pair (a, b) for any two nodes a and b." This
is consistent with the API's claim of constant-time edge existence testing, and
O(|V| + |E|) space usage.
