pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, max: i32) {
        if current.len() as i32 == max * 2 {
            result.push(current);
            return;
        }

        if open < max {
            backtrack(result, format!("{}(", current), open + 1, close, max);
        }
        if close < open {
            backtrack(result, format!("{})", current), open, close + 1, max);
        }
    }

    let mut result = Vec::new();
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }
}
