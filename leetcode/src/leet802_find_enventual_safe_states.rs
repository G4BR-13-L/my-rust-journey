


pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut safe_nodes: Vec<i32> = Vec::new();

    pub fn deep_first_search(node: i32, graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>) -> bool{
        if colors[node as usize] > 0 {
            return colors[node as usize] == 2;
        }
        colors[node as usize] = 1;
        for &neighbor in graph[node as usize].iter() {
            if !deep_first_search(neighbor, &graph, colors) {
                return false;
            }
        }
        colors[node as usize] = 2;
        return true;
    }

    let n = graph.len();
    let mut colors = vec![0; n];
    let mut result = Vec::new();
    for i in 0..n {
        if deep_first_search(i as i32, &graph, &mut colors) {
            result.push(i as i32);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let expected_result = vec![2, 4, 5, 6];
        assert_eq!(eventual_safe_nodes(graph), expected_result);
    }

    #[test]
    fn test_case2() {
        let graph = vec![
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![3, 4],
            vec![0, 4],
            vec![],
        ];
        let expected_result = vec![4];
        assert_eq!(eventual_safe_nodes(graph), expected_result);
    }

    #[test]
    fn test_case3() {
        let graph = vec![
            vec![],
            vec![0, 2, 3, 4],
            vec![3],
            vec![4],
            vec![],
        ];
        let expected_result = vec![0, 1, 2, 3, 4];
        assert_eq!(eventual_safe_nodes(graph), expected_result);
    }
}