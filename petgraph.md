# petgraph Review

`petgraph` is a very featureful library for weighted, labelled graphs, directed 
or undirected. Directedness is indicated at the type level, which is cool.

The major weakness is that it exposes nodes and edges to the user as indices,
which are checked for validity at runtime only and can cause panics. 
Worse, in the main `Graph` type, edge and vertex deletions cause index 
invalidation. 

At a high level, `petgraph` offers three
graph structures:
- `Graph`: basic graph, implemented with adjacency list (probably just Vecs, 
which they just .remove() from when vertices/edges are deleted -- this would
explain the unstable indices)
- `StableGraph`: also an adjacency list graph, but indices are stable after
removals. This probably means that they either use HashMaps instead of Vecs, or 
use Vecs and have a special "Deleted" marker to preserve indices.
- there's also `GraphMap`, which I didn't examine in too much detail I'm afraid