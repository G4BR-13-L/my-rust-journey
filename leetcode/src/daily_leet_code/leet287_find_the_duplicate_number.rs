pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut tartaruga: i32 = nums[0];
    let mut lebre: i32 = nums[0];
    let mut cycle: usize = 0;

    loop {
        cycle += 1;
        tartaruga = nums[tartaruga as usize];
        lebre = nums[nums[lebre as usize] as usize];

        if tartaruga == lebre {
            tartaruga = nums[0];
            while tartaruga != lebre {
                tartaruga = nums[tartaruga as usize];
                lebre = nums[lebre as usize];
            }
            return tartaruga;
        }

        if tartaruga == -1 || lebre == -1 {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_duplicate;

    #[test]
    fn sample_tests() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(find_duplicate(vec![3, 3, 3, 3, 3]), 3);
        assert_eq!(find_duplicate(vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1]), 9);

        // Additional tests
        assert_eq!(find_duplicate(vec![1, 1, 2, 3, 4]), 1); // Duplicate at the beginning
        assert_eq!(find_duplicate(vec![1, 2, 3, 4, 4]), 4); // Duplicate at the end
        assert_eq!(find_duplicate(vec![1, 2, 3, 3, 4]), 3); // Duplicate in the middle
        assert_eq!(find_duplicate(vec![1, 2, 2, 3, 3, 3, 4, 4, 4]), 2); // Multiple duplicates
    }
}
