mod floyd_warshall;
mod graph;
mod prim;

pub use graph::{Graph, Vertex, Edge, WeightedEdge, Vertices, Edges};

pub mod algorithm {
    //! Collection of graph algorithms.

    pub use floyd_warshall::floyd_warshall as all_pairs_shortest_path;
    pub use floyd_warshall::ShortestPaths;
    pub use prim::prim as minimum_spanning_tree;
}
