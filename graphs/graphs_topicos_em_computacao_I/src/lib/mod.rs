extern crate petgraph;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
use std::f64;

pub(crate) fn count_triangles(graph: &DiGraph<usize, ()>) -> usize {
    let mut total_triangles = 0;

    for node in graph.node_indices() {
        let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();

        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if graph.contains_edge(neighbors[i], neighbors[j]){
                    total_triangles += 1;
                }
            }
        }
    }

    total_triangles / 3
}

pub(crate) fn cluster_coefficient(graph: &DiGraph<usize, ()>, node: NodeIndex) -> f64 {
    let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();
    let k = neighbors.len();

    if k < 2 {
        return 0.0;
    }

    let mut connected_neighbors = 0;

    for i in 0..k {
        for j in (i + 1)..k {
            if graph.contains_edge(neighbors[i], neighbors[j]) {
                connected_neighbors += 1;
            }
        }
    }

    let triangles = count_triangles(graph);

    if triangles == 0 {
        return 0.0;
    }

    (2.0 * connected_neighbors as f64) / (k * (k - 1)) as f64
}

pub(crate) fn individual_path_length(graph: &DiGraph<usize, ()>) -> Vec<(NodeIndex, f64)> {
    let mut individual_lengths = Vec::new();

    for start_node in graph.node_indices() {
        let distances = dijkstra(graph, start_node, None, |_| 1);

        let mut total_length = 0.0;
        let mut total_pairs = 0;

        for (_, &distance) in distances.iter() {
            if distance as f64 != f64::INFINITY {
                total_length += distance as f64;
                total_pairs += 1;
            }
        }

        let avg_length = if total_pairs != 0 {
            total_length / total_pairs as f64
        } else {
            0.0
        };

        individual_lengths.push((start_node, avg_length));
    }

    individual_lengths
}

pub(crate) fn individual_density(graph: &DiGraph<usize, ()>) -> Vec<(NodeIndex, f64)> {
    let mut individual_densities = Vec::new();

    for node in graph.node_indices() {
        let num_edges = graph.edges(node).count() + graph.edges_directed(node, petgraph::Direction::Incoming).count();
        let num_nodes = graph.node_count();

        let density = if num_nodes <= 1 {
            0.0
        } else {
            (2 * num_edges) as f64 / (num_nodes * (num_nodes - 1)) as f64
        };

        individual_densities.push((node, density));
    }

    individual_densities
}