1. High-level features
2. Safety analysis
3. Strong and weak points
4. Internal representation

1. petgraph provides multiple different implementations of graphs with different properties, that each implement a basic set of graph traits. It provides CSR (Compresses Sparse Row) graphs, which seem to represent the graph as an adjacency matrix. It also allows you to store your graph as a Graph, where each vertex has an adjacency matrix, which stores the weight of each edge. The third possible representation is a GraphMap, which stores the nodes as keys in a HashTable for quick lookup.
Each graph implementation also implements a set of traits that allow the creation of iterators over different parts of the graph. It is possible to get an iterator over all of the edges of a vertex, or over all of the vertices of the graph. There are also obviously functions that allow adding vertices and edges. It is templated in such a way as to allow the edge weights to be an arbitrary type, as well as the vertices themselves to hold whatever type you choose. Additionally, it is templated on being directed or indirected, as desired.
petgraph also provides various graph algorithms that (mostly) work on all of the graph types included in the crate. They include shortest-path algorithms, as well as algorithms that find cycles and strongly-connected components.
petgraph also includes a graph display library.

2. The graph algorithms themselves are safe, returing Results, and making sure that they catch themselves if there is an infinite cycle.
A CSR graph panics when told to interact with nodes or edges that do not exist, as well as nodes or edges that are outside the bounds of the array.
A Graph will panic when adding edges to nodes that don't exist, or when adding nodes or edges while at its maximum size. In general, graph functions that panic do so when they try to increase the size beyond maximum, or are told to interact with edges or nodes that do not exist.
A GraphMap does not panic as often, choosing instead to add nodes or edges that do not exist when it is told to interact with them.
In general, these panic conditions make sense. In C++ they would fall into the `undefined behavior' category, because it doesn't make sense to do operations on nodes that do not exist. In RUST, it's safer just to panic.

3. petgraph has a lot of different implementations of graphs, and allows you to choose the one with the best speed guarantees for your purpose. It uses the RUST trait system to make this approach elegant. Each of the graphs implements a set of graph traits that allows them to work with the algorithms and iterator methods without relying on the implementation details. It's also relatively easy to switch between implementations, since graphs can be built from edge lists, and each type of graph can be converted into an edge list.
Graphs also implement STL traits like Collection, allowing them to play nicely with other STL code.

petgraph allows the creation of parallel edges, as well as the creation of edges from a vertex to itself. It does provide a method that creates edges that cannot be parallel, but there's no easy way to check if parallel or self-edges exist, and while there are some cases where this is useful (FSMs), there are also many cases where it is invalid.

4. CSR tells us that it is internally represented as an adjacency matrix, i.e. a VxV matrix where V is the number of vertices, where the value at (x, y) tells you what the weight of the edge is from x to y if any.
Graph and StableGraph both seem like a list of vertices, each with a list of edges of the vertices that they connect to.
GraphMap is probably a HashMap of Nodes, each of which also has a list of related edges.