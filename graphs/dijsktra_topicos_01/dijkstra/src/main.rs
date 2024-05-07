use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};

fn main() {
    let adjacency_list: HashMap<&str, Vec<&str>> = [
        ("A", vec!["B", "C"]),
        ("B", vec!["A", "C", "D"]),
        ("C", vec!["A", "B", "D"]),
        ("D", vec!["B", "C"]),
    ]
    .iter()
    .cloned()
    .collect();

    let mut graph = Graph::<&str, ()>::new();
    let mut node_indices = HashMap::<&str, NodeIndex>::new();

    for (node_name, _) in &adjacency_list {
        let node_index = graph.add_node(node_name);
        node_indices.insert(node_name, node_index);
    }

    for (source, targets) in &adjacency_list {
        let source_index = *node_indices.get(source).unwrap();
        for target in targets {
            let target_index = *node_indices.get(target).unwrap();
            graph.add_edge(source_index, target_index, ());
        }
    }

    let mut betweenness_centrality = HashMap::<&str, f64>::new();
    for (source_name, _) in &adjacency_list {
        let source_index = *node_indices.get(source_name).unwrap();
        let mut num_shortest_paths = HashMap::<NodeIndex, usize>::new();
        let mut predecessors = HashMap::<NodeIndex, HashSet<NodeIndex>>::new();
        let mut distances = HashMap::<NodeIndex, usize>::new();
        let mut stack = Vec::<NodeIndex>::new();
        let mut sigma = HashMap::<NodeIndex, f64>::new();

        for node_index in graph.node_indices() {
            num_shortest_paths.insert(node_index, 0);
            predecessors.insert(node_index, HashSet::new());
            distances.insert(node_index, usize::MAX);
            sigma.insert(node_index, 0.0);
        }

        distances.insert(source_index, 0);
        sigma.insert(source_index, 1.0);

        let mut queue = vec![source_index];
        while let Some(current) = queue.pop() {
            stack.push(current);
            for neighbor in graph.neighbors(current) {
                if distances[&neighbor] == usize::MAX {
                    queue.push(neighbor);
                    distances.insert(neighbor, distances[&current] + 1);
                }
                if distances[&neighbor] == distances[&current] + 1 {
                    sigma.insert(neighbor, sigma[&neighbor] + sigma[&current]);
                    predecessors.get_mut(&neighbor).unwrap().insert(current);
                }
            }
        }

        while let Some(current) = stack.pop() {
            for &predecessor in &predecessors[&current] {
                let delta = (sigma[&predecessor] / sigma[&current]) * (1.0 + betweenness_centrality.get(graph.node_weight(current).unwrap()).unwrap_or(&0.0));
                betweenness_centrality.entry(graph.node_weight(predecessor).unwrap()).and_modify(|e| *e += delta).or_insert(delta);
            }
        }
    }

    println!("Betweenness Centrality:");
    for (node_name, centrality) in &betweenness_centrality {
        println!("Node {}: {}", node_name, centrality);
    }
}
