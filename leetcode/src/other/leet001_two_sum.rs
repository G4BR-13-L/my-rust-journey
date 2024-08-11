use core::num;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if j != i && (nums[i] + nums[j]) == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return Vec::new();
}
#[cfg(test)]
mod tests {
    use crate::other::leet001_two_sum::two_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
        assert_eq!(two_sum(vec![3, 2, 3], 6), [0, 2]);
        assert_eq!(two_sum(vec![3,2,4], 6), [1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), [0, 1]);
    }
}
