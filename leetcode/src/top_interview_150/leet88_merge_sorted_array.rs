pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut nums3: Vec<i32> = Vec::with_capacity((m + n) as usize);
    let mut n_index: usize = 0;

    if n == 0 || nums2.len() == 0 {
        return;
    }
    for i in 0..(n + m) {
        let mut i_usize = i as usize;
        if nums1[i_usize] < nums2[n_index] && nums1[i_usize] != 0{
            nums3.push(nums1[i_usize]);
        }else if nums1[i_usize] == nums2[n_index]{
            nums3.push(nums1[i_usize]);
            nums3.push(nums2[n_index]);
            n_index+=1;
        } else if nums1[i_usize] > nums2[n_index]{
            nums3.push(nums2[n_index]);
            n_index += 1;
        }
    }
    *nums1 = nums3;
}