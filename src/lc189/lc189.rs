pub fn rotate(nums: &mut Vec<i32>, k: usize) {
    let l = nums.len();
    let temp: Vec<i32> = nums.iter().cloned().collect();

    let k = k % l;

    for i in 0..l {
        nums[(i + k) % l] = temp[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        let target = vec![5, 6, 7, 1, 2, 3, 4];
        assert_eq!(nums, target);
    }
}
