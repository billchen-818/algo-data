pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = num_map.get(&complement) {
            return vec![j, i as i32];
        }
        num_map.insert(num, i as i32);
    }
    vec![]
}

mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
