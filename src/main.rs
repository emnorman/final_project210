mod graph;
mod degree;
mod paths;
mod tests;

use std::io::{self, Write};

fn get_user_input(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<usize>().expect("Please enter a valid number")
}

fn main() {
    let path = "data/facebook_combined.txt";

    match graph::load_graph_from_file(path) {
        Ok(graph) => {
            println!("Graph loaded with {} nodes", graph.len());

            let degrees = degree::compute_degrees(&graph);
            println!("Average degree: {:.2}", degree::average_degree(&degrees));
            degree::print_top_degrees(&degrees, 5);

            let start = get_user_input("\nEnter starting node: ");
            let end = get_user_input("Enter target node: ");

            match paths::bfs_shortest_path(&graph, start, end) {
                Some(distance) => println!("\nShortest path from {} to {} is {} steps", start, end, distance),
                None => println!("\nNo path found between {} and {}", start, end),
            }

            println!("\nCalculating graph diameter... this may take a minute.");
            match paths::compute_diameter(&graph) {
                Some(diameter) => println!("Graph diameter: {}", diameter),
                None => println!("Could not compute diameter."),
            }

        }

        Err(e) => {
            eprintln!("Failed to load graph: {}", e);
        }
    }
}
