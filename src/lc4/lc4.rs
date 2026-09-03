pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums = nums1;
    nums.extend(nums2);
    nums.sort_unstable();
    let len = nums.len();
    if len % 2 == 0 {
        (nums[len / 2 - 1] as f64 + nums[len / 2] as f64) / 2.0
    } else {
        nums[len / 2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
