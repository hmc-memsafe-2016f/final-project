extern crate graph;
use graph::Graph;
use std::collections::HashSet;

pub fn dist_to_node<V>(g: &Graph<V,f64>, start: usize, end: usize) -> Option<f64>
{
    let mut distances = vec![std::f64::INFINITY; g.num_vertices()];
    distances[start] = 0.0;

    let mut unvisited: HashSet<_> = (0..start).chain(start+1..g.num_vertices()).collect();
    let mut current = start;
    while current != end {

        for (n,w) in g.neighbors_with_edge(current).filter(|&(n, _)| unvisited.contains(&n)) {
            let tdist = distances[current] + w;
            if tdist < distances[n] {
                distances[n] = tdist;
            }
        }

        let mut updated = false;
        for n in unvisited.iter() {
            if !updated || distances[*n] < distances[current] {
                current = n.clone();
                updated = true;
            }
        }

        unvisited.remove(&current);

        if distances[current] == std::f64::INFINITY {
            return None;
        }
    }
    Some(distances[end])
}

#[cfg(test)]
mod tests {
    use graph::Graph;
    use dist_to_node;
    #[test]
    fn trivial() {
        let mut g: Graph<(), f64> = Graph::new();
        let v = g.add_vertex(());
        assert_eq!(Some(0.0), dist_to_node(&g, v, v));
    }

    #[test]
    fn one_edge() {
        let mut g = Graph::new();
        let v1 = g.add_vertex(());
        let v2 = g.add_vertex(());
        g.add_edge(v1, v2, 42.0);
        assert_eq!(Some(0.0), dist_to_node(&g, v1, v1));
        assert_eq!(Some(0.0), dist_to_node(&g, v2, v2));
        assert_eq!(Some(42.0), dist_to_node(&g, v1, v2));
    }

    #[test]
    fn path() {
        let mut g = Graph::new();
        let start = g.add_vertex(());
        let mut pv = start;
        for i in 0..10 {
            let v = g.add_vertex(());
            // Make the edge backwards (shouldn't matter, it's undirected)
            g.add_edge(v, pv, i as f64);
            pv = v;
        }
        let end = g.add_vertex(());
        g.add_edge(pv, end, 42.0);

        assert_eq!(Some(87.0), dist_to_node(&g, start, end));
    }

    #[test]
    fn tree() {
        fn make_tree(g: &mut Graph<(), f64>, root: usize, depth: usize) {
            if depth > 0 {
                let child1 = g.add_vertex(());
                make_tree(g, child1, depth - 1);
                let child2 = g.add_vertex(());
                make_tree(g, child2, depth - 1);
                let child3 = g.add_vertex(());
                make_tree(g, child3, depth - 1);

                g.add_edge(root, child1, depth as f64);
                g.add_edge(root, child2, depth as f64);
                g.add_edge(root, child3, depth as f64);
            }
        }
        let mut g = Graph::new();
        let root = g.add_vertex(());
        make_tree(&mut g, root, 6);

        assert_eq!(Some(21.0), dist_to_node(&g, root, 491));
        assert_eq!(Some(21.0), dist_to_node(&g, root, 492));
    }

    #[test]
    fn cycle() {
        let mut g = Graph::new();
        let start = g.add_vertex(());
        let mut pv = start;
        for i in 0..10 {
            let v = g.add_vertex(());
            // Make the edge backwards (shouldn't matter, it's undirected)
            g.add_edge(v, pv, i as f64);
            pv = v;
        }

        assert_eq!(Some(45.0), dist_to_node(&g, pv, start));

        g.add_edge(pv, start, 2.0);

        assert_eq!(Some(2.0), dist_to_node(&g, pv, start));
    }

    #[test]
    fn disconnected_simple() {
        let mut g = Graph::new();
        let v1 = g.add_vertex(());
        let v2 = g.add_vertex(());
        assert_eq!(Some(0.0), dist_to_node(&g, v1, v1));
        assert_eq!(Some(0.0), dist_to_node(&g, v2, v2));
        assert_eq!(None, dist_to_node(&g, v1, v2));
    }

}
