extern crate final_project;
extern crate rand;
extern crate petgraph;

// much of the boilerplate here is copypasta from homework 4, 

// This macro is an assertion with nicely formatted failure output
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nExpected `{:?}` is not equal to Actual `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

// test the pairing heap
mod pairing_heap {

    use final_project::pairing_heap::PairingHeap;
    use rand::random;

    static TEST_SIZES: [usize; 4] = [1, 10, 100, 1000];

    fn random_vec(size: usize) -> Vec<usize> {
        let mut v = Vec::new();
        for _ in 0..size {
            // mod by size/2 to make sure we get duplicates
            v.push(random::<usize>() % (if size > 1 {size/2} else {size}));
        }
        v
    }
    
    #[test]
    fn test_basic() {
        for size in &TEST_SIZES {
            let mut heap = PairingHeap::<usize, ()>::new();
            let v = random_vec(*size);
            let sorted = { let mut copy = v.clone(); copy.sort(); copy };

            // generate a heap
            for i in &v {
                heap.insert(*i, ());
            }

            // drain the heap, make sure order is preserved
            for i in sorted {
                {
                    let min = heap.find_min();
                    assert!(min.is_some());
                    let (k,_) = min.unwrap();
                    assert_expected_eq_actual!(*k, i);
                    assert!(!heap.is_empty());
                }

                {
                    let min = heap.delete_min();
                    assert!(min.is_some());
                    let (k,_) = min.unwrap();
                    assert_expected_eq_actual!(k, i);
                }
            }
            assert!(heap.is_empty());
        }
    }

    #[test]
    fn test_remove_min() {
        for size in &TEST_SIZES {
            let mut heap = PairingHeap::<usize, ()>::new();
            let v = random_vec(*size);
            let mut updated = v.clone();
            let mut handles = Vec::new();

            // generate a heap
            for i in &v {
                handles.push(heap.insert(*i, ()));            
            }

            // decrease all the keys in the heap by a random amount
            for (i,h) in handles.iter().enumerate() {
                // pick a new, smaller key
                let new = if v[i] == 0 {
                    v[i]
                } else {
                    v[i] - (random::<usize>() % v[i])
                };
                updated[i] = new;
                heap.update_key(h, new);
            }
            updated.sort();

            // drain the heap and ensure that it reflects all the updated keys
            for i in updated {
                {
                    let min = heap.find_min();
                    assert!(min.is_some());
                    let (k,_) = min.unwrap();
                    assert_expected_eq_actual!(*k, i);
                    assert!(!heap.is_empty());
                }

                {
                    let min = heap.delete_min();
                    assert!(min.is_some());
                    let (k,_) = min.unwrap();
                    assert_expected_eq_actual!(k, i);
                }
            }
            assert!(heap.is_empty());
        }
    }
}

mod graph {

    use final_project;
    use petgraph;
    use petgraph::visit::EdgeRef;
    use petgraph::graph::NodeIndex;

    #[test]
    fn test_shortest_path_k4() {
        // K_4 with edge weights of 1, taken from petgraph docs
        let gr = petgraph::Graph::<(), usize>::from_edges(&[
            (0, 1, 1), (0, 2, 1), (0, 3, 1),
            (1, 2, 1), (1, 3, 1),
            (2, 3, 1),
            ]);

        let mut my_gr = final_project::Graph::new(gr.node_indices(),
                                                  |n| {
                                                      gr.edges(n).map(|e| {
                                                          (e.target(), *e.weight())
                                                      })
                                                  });

        for e in gr.edge_indices() {
            let (src, dst) = gr.edge_endpoints(e).unwrap();
            assert_expected_eq_actual!(my_gr.shortest_path_len(src, dst), 1);
        }
    }

    #[test]
    fn test_shortest_path_c6() {
        // C_6 with one heavy edge
        let gr = petgraph::Graph::<(), usize>::from_edges(&[
            (0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1), (4, 5, 1), (5, 0, 10),
            ]);

        let mut my_gr = final_project::Graph::new(gr.node_indices(),
                                                  |n| {
                                                      gr.edges(n).map(|e| {
                                                          (e.target(), *e.weight())
                                                      })
                                                  });

        for i in 1..6 {
            assert_expected_eq_actual!(
                my_gr.shortest_path_len(NodeIndex::new(0), NodeIndex::new(i)), i);
        }
    }

    #[test]
    // ths test fails, i.e. our MST implementation does not work. 
    fn test_mst_c6() {
        // C_6 with one heavy edge
        let gr = petgraph::Graph::<(), usize>::from_edges(&[
            (0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1), (4, 5, 1), (5, 0, 10),
            ]);

        let mut my_gr = final_project::Graph::new(gr.node_indices(),
                                                  |n| {
                                                      gr.edges(n).map(|e| {
                                                          (e.target(), *e.weight())
                                                      })
                                                  });

        let mst = my_gr.spanning_tree();
        assert_expected_eq_actual!(mst.len(), 6); // we better have 6 verticies
        let mut found_root = false;
        for (i, adj) in (*mst).iter().enumerate() {
            if adj.len() == 0 {
                assert!(!found_root);
                found_root = true;
            } else {
                let parent = *adj.first().unwrap();
                if i == 5 {
                    assert!(parent == NodeIndex::new(4));
                } else if i == 0 {
                    assert!(parent == NodeIndex::new(1));
                } else {
                    assert!(parent == NodeIndex::new((i + 1)%6)
                        || parent == NodeIndex::new((i - 1)%6));
                }
            }
        }
    }
}
