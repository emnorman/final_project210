use std::collections::{HashMap, VecDeque};
pub type Graph = HashMap<usize, Vec<usize>>;

pub fn bfs_shortest_path(graph: &Graph, start: usize, target: usize) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if current == target {
            return visited.get(&current).copied();
        }

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, visited[&current] + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    None // No path found
}

pub fn compute_diameter(graph: &Graph) -> Option<usize> {
    let mut max_distance = 0;

    for &start in graph.keys() {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(start, 0);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains_key(&neighbor) {
                        let dist = visited[&current] + 1;
                        visited.insert(neighbor, dist);
                        queue.push_back(neighbor);
                        if dist > max_distance {
                            max_distance = dist;
                        }
                    }
                }
            }
        }
    }

    if max_distance > 0 {
        Some(max_distance)
    } else {
        None
    }
}
