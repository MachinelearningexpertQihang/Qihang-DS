use std::collections::{HashMap, HashSet};

pub struct EdgeWeightedDigraph {
    pub graph: Vec<Vec<i32>>,
}

impl EdgeWeightedDigraph {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![0; n]; n],
        }
    }

    pub fn add_weight(&mut self, start: usize, end: usize, weight: i32) {
        self.graph[start][end] += weight;
    }

    pub fn get_weight(&self, start: usize, end: usize) -> i32 {
        self.graph[start][end]
    }

    pub fn degree_centrality(&self, type_str: &str) -> Vec<(usize, usize)> {
        let mut degrees = vec![];

        for i in 0..self.graph.len() {
            let in_degree: usize = self.graph.iter().map(|row| row[i] as usize).sum();
            let out_degree: usize = self.graph[i].iter().map(|&weight| weight as usize).sum();
            let degree = match type_str {
                "in-degree" => in_degree,
                "out-degree" => out_degree,
                "combined" => in_degree + out_degree,
                _ => panic!(
                    "Invalid type '{}', must be 'in-degree', 'out-degree', or 'combined'",
                    type_str
                ),
            };

            degrees.push((i, degree));
        }

        degrees
    }

    pub fn simple_betweenness_centrality(&self) -> Vec<(usize, usize)> {
        let mut centrality = HashMap::new();

        for i in 0..self.graph.len() {
            let mut visited = HashSet::new();
            let mut queue = vec![i];
            visited.insert(i);

            while !queue.is_empty() {
                let node = queue.remove(0);
                for j in 0..self.graph[node].len() {
                    if self.graph[node][j] > 0 && !visited.contains(&j) {
                        visited.insert(j);
                        queue.push(j);
                        *centrality.entry(j).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut centrality_vec: Vec<(usize, usize)> = centrality.into_iter().collect();
        centrality_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        centrality_vec
    }
}

pub struct PageRank {
    pub graph: EdgeWeightedDigraph,
    pub damping_factor: f64,
    pub iterations: usize,
}

impl PageRank {
    pub fn new(graph: EdgeWeightedDigraph, damping_factor: f64, iterations: usize) -> Self {
        Self {
            graph,
            damping_factor,
            iterations,
        }
    }

    pub fn run(&self) -> Vec<(usize, f64)> {
        let num_nodes = self.graph.graph.len();
        let uniform_pagerank = 1.0 / num_nodes as f64;
        let mut pagerank: Vec<f64> = vec![uniform_pagerank; num_nodes];

        for _ in 0..self.iterations {
            let mut new_pagerank: Vec<f64> = vec![0.0; num_nodes];

            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if self.graph.get_weight(j, i) > 0 {
                        let out_degree_j = self.graph.graph[j].iter().sum::<i32>() as f64;
                        new_pagerank[i] += pagerank[j] / out_degree_j;
                    }
                }
            }

            for i in 0..num_nodes {
                new_pagerank[i] =
                    (1.0 - self.damping_factor) / num_nodes as f64 + self.damping_factor * new_pagerank[i];
            }

            pagerank = new_pagerank.clone();
        }

        let pagerank_vec: Vec<(usize, f64)> = pagerank.into_iter().enumerate().collect();
        let mut sorted_pagerank = pagerank_vec.clone();
        sorted_pagerank.sort_by(|(_, rank1), (_, rank2)| rank2.partial_cmp(rank1).unwrap());

        sorted_pagerank
    }
}