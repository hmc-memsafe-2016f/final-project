mod floyd_warshall;
mod graph;
mod prim;

pub use graph::{Graph,Vertex,Edges,Edge};

pub mod algorithm { 
    pub use floyd_warshall::floyd_warshall as all_pairs_shortest_path;
    // pub use prim::prim as minimum_spanning_tree;
}