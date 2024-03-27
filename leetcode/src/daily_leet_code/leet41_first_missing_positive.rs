use std::f32::INFINITY;

const INF: i32 = i32::max_value();

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if nums[i] <= 0 || nums[i] > nums.len() as i32 {
            nums[i] = INF;
        }
    }

    for i in 0..nums.len() {
        let index = nums[i].abs() - 1;
        if index != INF - 1 {
            nums[index as usize] = -nums[index as usize].abs();
        }
    }

    for i in 0..nums.len() {
        if nums[i].is_positive() {
            return i as i32 + 1;
        }
    }
    return nums.len() as i32 + 1;
}


#[cfg(test)]
mod tests {
    use super::first_missing_positive;

    #[test]
    fn case1() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn case4() {
        assert_eq!(first_missing_positive(vec![2, 1]), 3);
    }

    #[test]
    fn case5() {
        assert_eq!(first_missing_positive(vec![1, 1]), 2);
    }
}