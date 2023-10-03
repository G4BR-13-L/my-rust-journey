// Given an array of integers.

// Return an array, where the first element is the count of positives numbers and the second element is sum of negative numbers. 0 is neither positive nor negative.

// If the input is an empty array or is null, return an empty array.
// Example

// For input [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], you should return [10, -65].

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 { return vec![] }
    let mut count_positives: i32 = 0;
    let mut sum_negatives: i32 = 0;
    for i in input {
        if i >= 1 {
            count_positives+=1;
        }
        if i <= -1 {
            sum_negatives += i;
        }
    }
    return vec![count_positives, sum_negatives]
}

#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;
        
    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert!(actual == expected, 
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
        dotest(&[], &[]);
        dotest(&[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14], &[8, -50]);
        dotest(&[0,1,2,3,4,5], &[5, 0]);
        dotest(&[1,2,3,4,5], &[5, 0]);
        dotest(&[0,-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[0,0,0,0], &[0,0]);
        dotest(&[0], &[0,0]);
    }
}
