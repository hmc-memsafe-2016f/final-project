
# Graph and Algorithm Implementation

## Kruskal's Algorithm

### Inputs
An undirected weighted graph, `G`.

### Outputs
A collection of edges such that the induced subgraph of `G` using
those edges is a minimum weight spanning forest of `G`.

### Process of Computation
Use a union-find data structure (described in Algs) in which every
node begins in their own set, and we repeatedly union sets of forests
until we either have a spanning tree for `G` or we have a spanning tree
for each connected component of `G`.

## Parallelized Maximal Independent Set

### Inputs
An undirected weighted graph, `G`.

### Outputs
A collection of nodes `S` such that no two nodes in `S` are adjacent
and and node in `G` is either adjacent to `S` or in `S`.

### Process of Computation
From the High Performance Computing Class in Fall 2015, we learned
a method of parallelizing the Maximum Independent Set problem.
The algorithm is described as follows:

 * Label each vertex with an arbitrary and distinct integer.
 * Make a data structure of possible vertices to be added, initialize with all vertices.
 * In parallel, select vertices from this structure, for each chosen vertex:
   * Flag it for addition to the final set.
   * If it has a neighbor in the canidate set with a higher id flagged for additon, unflag self.
   * If we are still flagged for addition, flag it and remove all neighbors and self from candidate set.

(Sync points set appropriately.)

