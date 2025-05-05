use std::collections::HashMap;
use crate::graph;
use crate::paths;

#[test]
fn test_graph_loading_small_sample() {
    let data = "0 1\n1 2\n2 3\n3 0";
    let file_path = "test_graph.txt";

    // Write temporary file
    std::fs::write(file_path, data).expect("Failed to write test file");

    let graph = graph::load_graph_from_file(file_path).expect("Failed to load graph");

    assert_eq!(graph.len(), 4); // nodes 0–3
    assert_eq!(graph.get(&0).unwrap().len(), 2); // 0 is connected to 1 and 3

    // Clean up
    std::fs::remove_file(file_path).unwrap();
}

#[test]
fn test_bfs_shortest_path_simple() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![1, 3]);
    graph.insert(3, vec![2]);

    let result = paths::bfs_shortest_path(&graph, 0, 3);
    assert_eq!(result, Some(3));
}
