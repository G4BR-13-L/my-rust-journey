extern crate petgraph;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
use std::f64;

fn count_triangles(graph: &DiGraph<usize, ()>) -> usize {
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

fn cluster_coefficient(graph: &DiGraph<usize, ()>, node: NodeIndex) -> f64 {
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

fn individual_path_length(graph: &DiGraph<usize, ()>) -> Vec<(NodeIndex, f64)> {
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

fn individual_density(graph: &DiGraph<usize, ()>) -> Vec<(NodeIndex, f64)> {
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

fn main() {
    let mut graph = DiGraph::new();

    let node1 = graph.add_node(1);
    let node2 = graph.add_node(2);
    let node3 = graph.add_node(3);
    let node4 = graph.add_node(4);
    let node5 = graph.add_node(5);
    let node6 = graph.add_node(6);
    
    graph.add_edge(node1, node3, ());
    graph.add_edge(node1, node5, ());
    graph.add_edge(node1, node6, ());

    graph.add_edge(node2, node5, ());
    graph.add_edge(node2, node6, ());

    graph.add_edge(node3, node4, ());
    graph.add_edge(node3, node5, ());

    graph.add_edge(node4, node5, ());
    graph.add_edge(node4, node6, ());

    graph.add_edge(node5, node6, ());

    println!("Total de triângulos no grafo (total of triangles): {}", count_triangles(&graph));

    for node in graph.node_indices() {
        let coefficient = cluster_coefficient(&graph, node);
        println!("Coeficiente de cluster do nó {}(Cluster Coeficient): {}", node.index(), coefficient);
    }

    let individual_lengths = individual_path_length(&graph);
    println!("Comprimento médio do caminho para cada vértice (Average length for each node):");
    for (node, length) in &individual_lengths {
        println!("Nó {}: {}", node.index(), length);
    }
    let total_avg_path_length: f64 = individual_lengths.iter().map(|(_, length)| length).sum();
    println!("Comprimento médio do caminho total no grafo(Average graph length): {}", total_avg_path_length);

    let individual_densities = individual_density(&graph);
    println!("Densidade da rede para cada vértice (Node Subgraph density):");
    for (node, density) in &individual_densities {
        println!("Nó {}: {}", node.index(), density);
    }
    let total_density: f64 = individual_densities.iter().map(|(_, density)| density).sum();
    println!("Densidade total da rede(Network Desnity): {}", total_density);
}
