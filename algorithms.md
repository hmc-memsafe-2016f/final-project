##Two Algorithms I want to Implement

###Prim's

Prim's algorithm takes in a graph and outputs the minimum spanning tree of a
weighted undirected graph. This means that Prim's algorithm finds the subset of
edges that connects all vertices in the graph, and has the minimal total weight.

Prim's algorithm will take an input of a graph, and output a set or list of
edges from that graph. The returned set of edges would be the minimum spanning
tree.

Prim's algorithm works as follows. At the beginning, the algorithm selects an
arbitrary vertex. Then it will select the edge with the least weight that
connects from a vertex that has been selected to a vertex that has not yet been
selected. Repeatedly select new edges (the previous step) until all vertices in
the graph have been selected. When all vertices have been selected, return the
set of edges that were selected to make this happen

The algorithm might keep track of vertices that have been selected by either
marking the vertices as "selected" or adding vertices to a set "selected". The
algorithm will also add the edges that were needed to select the new vertices to
a different set "edges". "Edges" is the set that will be ultimately returned
when the algorithm terminates.

###Kruskal's

Similar to Prim's, Kruskal's algorithm is also an algorithm to find the
minimum spanning tree of a graph.

Kruskal's algorithm takes in a graph and returns the set of edges that create a
minimum spanning tree.

Kruskal's algorithm works as follows. Create a set E of all edges in the graph,
then create a set of trees Forrest - each vertex in the graph should be its own
tree at the moment. Then, while E is nonempty and Forrest still not spanning
(spanning means either: if it's a connected graph, the Forrest has one component;
if it's not a connect graph, then Forrest has the number of trees equal to number
of separate components in the graph), get the smallest edge from E (popping it
from E as well). If that edge connects two trees together, then connect those
trees and put that edge into a set of edges called "used". After the while loop
terminates, return the set of edges "used".

