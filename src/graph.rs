use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub type Graph = HashMap<usize, Vec<usize>>;

pub fn load_graph_from_file(path: &str) -> Result<Graph> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph: Graph = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        let u: usize = parts[0].parse().unwrap_or(0);
        let v: usize = parts[1].parse().unwrap_or(0);

        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    Ok(graph)
}

