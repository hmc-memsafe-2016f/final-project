## Petgraph Library Analysis

#### Summary

`petgraph` is a Rust library of graph data structures and algorithms.
It contains several graph representations and related data structures as well
as a suite of algorithms to operate on them.

#### Graph Representations

The `petgraph` library contains several representations of graphs:

1. `Graph`, an adjacency list, i.e. presumably a `Vec<Vec<...>> `

2. `StableGraph`, which does not invalidate references into itself when
   edges or vertices are removed, i.e. presumably a `Vec<Vec<Option<...>>>` with
   some careful removal procedures,

3. `GraphMap`, an adjacency list implemented with a hash map, i.e. presumably
   a `HashMap<Vertex, HashSet<Vertex>>` or similar.

4. `Csr`, a sparse adjacency matrix.

#### API Quality

All of these graphs are relatively rich APIs that support basic opperations
like vertex and edge insertion and removal as well as more complicated
operations like updating weights, applying functions, etc. The richness of
the APIs are a strong point of this library.

The APIs are somewhat unfriendly in that many of the functions `panic()`
if the user provides invalid arguments instead of doing something more
friendly like returning an `Option` or a `Result`.

#### Other Data Structures

`petgraph` also contains a union-find data structure.

