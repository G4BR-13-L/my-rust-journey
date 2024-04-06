// Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.

// Example 1:

// Input: nums = [10,5,2,6], k = 100
// Output: 8
// Explanation: The 8 subarrays that have product less than 100 are:
// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
// Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
// Example 2:

// Input: nums = [1,2,3], k = 0
// Output: 0

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n: usize = nums.len();
    let mut esquerda: usize = 0;
    let mut direita: usize = 0;
    let mut atual: i32 = 1;
    let mut contagem: i32 = 0;

    while direita < n {
        if (nums[direita] >= k) {
            atual = 1;
            direita += 1;
            esquerda = direita;
            continue;
        }
        
        atual *= nums[direita];

        while (atual >= k) {
            atual /= nums[esquerda];
            esquerda += 1;
        }

        contagem += (direita - esquerda + 1) as i32;

        direita += 1;
    }

    contagem
}
