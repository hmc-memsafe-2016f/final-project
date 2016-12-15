# Graph Algorithms
The two algorithms I intend to implement are Dijkstra's Algorithm and the VF2 Algorithm.

## [Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
Dijkstra's Algorithm is a single-source shortest-path algorithm; given a start-node,
it finds the shortest (a.k.a. least-cost) path to each other node in the graph.
It is designed for directed graphs; an undirected graph can be transformed to a
directed graph by simply replacing each undirected edge with two directed edges, one
in each direction.

The algorithm itself is rather simple. A priority queue, implemented as a heap,
is constructed with all of the nodes besides the start-node, and each of them is
given a score of infinity. We "lock" our start node, and make "relaxation offers"
to all of its neighbors; essentially we look at the edge weight of each of the edges,
and ask its neighbors if the offered weight is better than its current score, and if
so we change the value and fix our heap. Once we've made all of the relaxation offers
we look at the first member of our heap and "lock" that one, and repeat the process.
Eventually, we've locked in all of the nodes and we know the least cost to each other node.

This algorithm assumes no negative-weight edges, and like all shortest-path algorithms
cannot handle negative cost cycles.

## [VF2](https://www.researchgate.net/publication/3193784_A_SubGraph_Isomorphism_Algorithm_for_Matching_Large_Graphs)
VF2 is a fast (sub)graph isomorphism algorithm. It basically constructs a search
tree for isomorphisms, and performs a DFS on that tree. It starts by comparing the empty
graph with the empty graph, which is obviously correct. Then it picks an arbitrary node
and tries to match it to an arbitrary node in the other graph (which will always succeed
as long as the graphs are both non-empty). It then picks a neighbor of the first and matches
it to a neighbor of the partner in the other graph, and so on. If at any point a node
can't be matched, the algorithm backs up a step and tries another neighbor.
In this fashion it iteratively searches through all the possible matches.

VF2 itself is not especially fast, however it has been the inspiration for several
additional improvements in the field, for example [VF2 Plus](https://www.researchgate.net/publication/276276829_VF2_Plus_An_Improved_version_of_VF2_for_Biological_Graphs)
and [VF2++](https://ecmiindmath.org/2016/04/11/vf2-a-subgraph-isomophism-algorithm-for-molecular-pattern-matching/).
