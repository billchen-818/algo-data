pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut candidates = candidates;
    candidates.sort();
    backtrack(&candidates, target, 0, &mut current, &mut result);
    result
}

fn backtrack(
    candidates: &[i32],
    target: i32,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        result.push(current.clone());
        return;
    }
    for i in start..candidates.len() {
        if candidates[i] > target {
            break;
        }
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }
        current.push(candidates[i]);
        backtrack(candidates, target - candidates[i], i + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = combination_sum2(candidates, target);
        let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(result, expected);
    }
}
