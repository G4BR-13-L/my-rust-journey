pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    fn get_vertice_inexplorado(td: &[i32]) -> Option<usize> {
        td.iter().position(|&x| x == 0)
    }

    fn existe_vertice_inexplorado(td: &[i32]) -> bool {
        td.iter().any(|&x| x == 0)
    }

    fn busca_em_profundidade(v: usize, rooms: &Vec<Vec<i32>>, t: &mut i32, td: &mut Vec<i32>, tt: &mut Vec<i32>, pai: &mut Vec<i32>) {
        *t += 1;
        td[v] = *t;
        for &w in &rooms[v] {
            if td[w as usize] == 0 {
                pai[w as usize] = v as i32;
                busca_em_profundidade(w as usize, rooms, t, td, tt, pai);
            }
        }
        *t += 1;
        tt[v] = *t;
    }

    let n = rooms.len();
    let mut t: i32 = 0;
    let mut td = vec![0; n];
    let mut tt = vec![0; n];
    let mut pai = vec![0; n];

    while existe_vertice_inexplorado(&td) {
        if let Some(v) = get_vertice_inexplorado(&td) {
            busca_em_profundidade(v, &rooms, &mut t, &mut td, &mut tt, &mut pai);
        }
        if existe_vertice_inexplorado(&td){
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case1() {
        let graph = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![],
        ];
        let expected_result = true;
        assert_eq!(can_visit_all_rooms(graph), expected_result);
    }

    #[test]
    fn test_case2() {
        let graph = vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0],
        ];
        let expected_result = false;
        assert_eq!(can_visit_all_rooms(graph), expected_result);
    }

    #[test]
    fn test_case3() {
        let graph = vec![
            vec![1],
            vec![1],
        ];
        let expected_result = true;
        assert_eq!(can_visit_all_rooms(graph), expected_result);
    }
}