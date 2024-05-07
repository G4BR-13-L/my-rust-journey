use std::collections::{HashMap, HashSet};
use std::usize::MAX;

struct Graph {
    edges: HashMap<char, Vec<char>>,
}

impl Graph {
    fn dijkstra(&self, start_node: char) -> HashMap<char, usize> {
        let mut distances: HashMap<char, usize> = HashMap::new();
        let mut visited: HashSet<char> = HashSet::new();
        let mut queue: Vec<char> = vec![start_node];

        for &node in self.edges.keys() {
            distances.insert(node, MAX);
        }
        distances.insert(start_node, 0);

        while let Some(current) = queue.pop() {
            visited.insert(current);
            if let Some(neighbors) = self.edges.get(&current) {
                for &neighbor in neighbors {
                    let new_distance = distances[&current] + 1;
                    if new_distance < *distances.get(&neighbor).unwrap_or(&MAX) {
                        distances.insert(neighbor, new_distance);
                        if !visited.contains(&neighbor) {
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }

        distances
    }
}

fn main() {
    let mut graph = Graph {
        edges: HashMap::new(),
    };

    graph.edges.insert('A', vec!['B', 'C']);
    graph.edges.insert('B', vec!['A', 'C', 'D']);
    graph.edges.insert('C', vec!['A', 'B', 'D']);
    graph.edges.insert('D', vec!['B', 'C']);

    let closeness_centralities = graph.dijkstra('A');
    println!("Closeness Centralities: {:?}", closeness_centralities);
}