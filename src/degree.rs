use std::collections::HashMap;

pub type Graph = HashMap<usize, Vec<usize>>;

pub fn compute_degrees(graph: &Graph) -> HashMap<usize, usize> {
    graph.iter()
        .map(|(node, neighbors)| (*node, neighbors.len()))
        .collect()
}

pub fn average_degree(degrees: &HashMap<usize, usize>) -> f64 {
    let total: usize = degrees.values().sum();
    let count = degrees.len();
    total as f64 / count as f64
}

pub fn print_top_degrees(degrees: &HashMap<usize, usize>, top_n: usize) {
    let mut sorted: Vec<(&usize, &usize)> = degrees.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1)); // descending order

    println!("\nTop {} nodes by degree:", top_n);
    for (node, degree) in sorted.iter().take(top_n) {
        println!("Node {}: Degree {}", node, degree);
    }
}
