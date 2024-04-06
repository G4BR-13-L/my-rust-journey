mod lib;

use petgraph::graph::DiGraph;
use lib::*;

pub fn process_graph(graph: &DiGraph<usize, ()>) -> () {


    let mut clustering_coefficient: f64 = 0 as f64;
    for node in graph.node_indices() {
        clustering_coefficient += cluster_coefficient(&graph, node);
    }
    let average_cluster_coeficient = clustering_coefficient / graph.node_count() as f64;
    println!("clustering coeficient: {}", count_triangles(&graph));

    let individual_lengths = individual_path_length(&graph);
    let total_avg_path_length: f64 = individual_lengths.iter().map(|(_, length)| length).sum();
    println!("Average distance: {}", total_avg_path_length);
}

fn main() {
    let mut graph1 = DiGraph::new();

    let graph1_node0 = graph1.add_node(0);
    let graph1_node1 = graph1.add_node(1);
    let graph1_node2 = graph1.add_node(2);
    let graph1_node3 = graph1.add_node(3);
    let graph1_node4 = graph1.add_node(4);
    let graph1_node5 = graph1.add_node(5);
    
    graph1.add_edge(graph1_node0, graph1_node2, ());
    graph1.add_edge(graph1_node0, graph1_node4, ());
    graph1.add_edge(graph1_node0, graph1_node5, ());
    graph1.add_edge(graph1_node1, graph1_node4, ());
    graph1.add_edge(graph1_node1, graph1_node5, ());
    graph1.add_edge(graph1_node2, graph1_node3, ());
    graph1.add_edge(graph1_node2, graph1_node4, ());
    graph1.add_edge(graph1_node4, graph1_node5, ());


    println!("\n\n---------------------------------------------------------------");
    println!("--------------------------- GRAPH A ---------------------------");
    process_graph(&graph1);

    println!("---------------------------------------------------------------");

    let mut graph2 = DiGraph::new();

    let graph2_node0 = graph2.add_node(0);
    let graph2_node1 = graph2.add_node(1);
    let graph2_node2 = graph2.add_node(2);
    let graph2_node3 = graph2.add_node(3);
    let graph2_node4 = graph2.add_node(4);
    let graph2_node5 = graph2.add_node(5);


    graph2.add_edge(graph2_node0, graph2_node3, ());
    graph2.add_edge(graph2_node0, graph2_node1, ());
    graph2.add_edge(graph2_node0, graph2_node2, ());
    graph2.add_edge(graph2_node0, graph2_node4, ());
    graph2.add_edge(graph2_node0, graph2_node5, ());
    graph2.add_edge(graph2_node1, graph2_node4, ());
    graph2.add_edge(graph2_node1, graph2_node2, ());
    graph2.add_edge(graph2_node1, graph2_node5, ());
    graph2.add_edge(graph2_node2, graph2_node4, ());
    graph2.add_edge(graph2_node2, graph2_node5, ());
    graph2.add_edge(graph2_node2, graph2_node3, ());
    graph2.add_edge(graph2_node3, graph2_node1, ());
    graph2.add_edge(graph2_node4, graph2_node5, ());


    println!("--------------------------- GRAPH B ---------------------------");
    process_graph(&graph2);
    
}
