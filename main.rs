mod pagerank;

use std::fs::File;
use std::io::{BufRead, BufReader};
use pagerank::{EdgeWeightedDigraph, PageRank};

fn main() {
    let mut graph = EdgeWeightedDigraph::new(168);

    let file = File::open("communication.csv").expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines().skip(1) {
        let s = line.unwrap();
        let values: Vec<&str> = s.split(';').collect();

        let start = values[0].parse::<usize>().unwrap();
        let end = values[1].parse::<usize>().unwrap();

        graph.add_weight(start, end,5);
    }

    let types = ["in-degree", "out-degree", "combined"];
    for &type_str in types.iter() {
        let degree_centralities = graph.degree_centrality(type_str);
        let mut sorted_nodes = degree_centralities.clone();
        sorted_nodes.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));

        println!("Top 5 nodes by number of emails ({})", type_str);
        for i in 0..5 {
            let (node, degree) = sorted_nodes[i];
            println!("Node {}: {}", node, degree);
        }
        println!();
    }

    let simple_centralities = graph.simple_betweenness_centrality();

    println!("Top 5 nodes by simple betweenness centrality:");
    for i in 0..5 {
        let (node, centrality) = simple_centralities[i];
        println!("Node {}: {}", node, centrality);
    }

    let var_name = PageRank::new(graph, 0.85, 100);
    let pagerank = var_name;
    let pagerank_result = pagerank.run();

    println!("Top 5 nodes by PageRank:");
    for i in 0..5 {
        let (node, rank) = pagerank_result[i];
        println!("Node {}: {:.6}", node, rank);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank() {
        // Create a small graph
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_weight(0, 1, 1);
        graph.add_weight(1, 2, 1);
        graph.add_weight(2, 3, 1);
        graph.add_weight(3, 0, 1);

        // Initialize PageRank algorithm with a damping factor of 0.85 and 100 iterations
        let pagerank = PageRank::new(graph, 0.85, 100);

        // Run the PageRank algorithm
        let pagerank_result = pagerank.run();

        // Expected PageRank values (these may vary based on the graph structure)
        let expected_pagerank = vec![(0, 0.25), (1, 0.25), (2, 0.25), (3, 0.25)];

        // Check if the actual PageRank values are close to the expected values
        for (node, rank) in expected_pagerank {
            let result = pagerank_result.iter().find(|(n, _)| *n == node).unwrap();
            assert!(
                (result.1 - rank).abs() < 1e-6,
                "PageRank mismatch for node {}: expected {}, actual {}",
                node,
                rank,
                result.1
            );
        }
    }
}
