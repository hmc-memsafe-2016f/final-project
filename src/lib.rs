mod floyd_warshall;
mod graph;

pub use graph::{Graph, VIndex, Edge};

pub mod algorithm {
    pub use floyd_warshall::floyd_warshall as all_pairs_shortest_path;
}
