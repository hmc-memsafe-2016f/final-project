# Algorithms

Prim's algorithm computes a minimum-weight spanning tree (set of edges) 
of an *undirected* graph. It's a greedy algorithm that builds up the tree
iteratively as follows:
- maintain a set of vertices "in the tree" (initialized to an arbitary vertex)
- repeatedly, add the minimum-weight edge in the graph that connects a vertex 
in the tree with a vertex not in the tree

Floyd-Warshall computes the shortest path between every pair of vertices of a 
graph. It is a dynamic programming algorithm. The essential idea is, for every 
pair of vertices (i,j), compute the shortest path between i and j that uses 
only the "first k" vertices as possible intermediate vertices on the path 
(the vertices are ordered arbitrarily for the "first k"). We repeatedly 
increase k by looking at the k-1 case (hence the dynamic programming) and the
kth vertex. 