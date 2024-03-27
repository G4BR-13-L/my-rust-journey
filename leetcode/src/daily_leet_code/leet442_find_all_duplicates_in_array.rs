use std::collections::HashMap;

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut duplicates = Vec::new();
    
    for &n in &nums {
        *map.entry(n).or_insert(0) += 1;
    }

    for (&num, &count) in &map {
        if count > 1 {
            duplicates.push(num);
        }
    }

    duplicates
}

#[cfg(test)]
mod tests {
    use super::find_duplicates;

    // #[test]
    // fn test1() {
    //     let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
    //     let expected = vec![2, 3];
    //     assert_eq!(find_duplicates(input.clone()), expected, "Input: {:?}, Expected: {:?}, Obtido: {:?}", input, expected, find_duplicates(input.clone()));
    // }
    //
    // #[test]
    // fn test2() {
    //     let input = vec![1, 1, 2];
    //     let expected = vec![1];
    //     assert_eq!(find_duplicates(input.clone()), expected, "Input: {:?}, Expected: {:?}, Obtido: {:?}", input, expected, find_duplicates(input.clone()));
    // }
    //
    // #[test]
    // fn test3() {
    //     let input = vec![1];
    //     let expected = vec![];
    //     assert_eq!(find_duplicates(input.clone()), expected, "Input: {:?}, Expected: {:?}, Obtido: {:?}", input, expected, find_duplicates(input.clone()));
    // }
}
