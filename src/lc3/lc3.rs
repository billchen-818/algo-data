pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_index_map = std::collections::HashMap::new();
    let mut start = 0;
    let mut max_length = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_index_map.get(&c) {
            start = start.max(prev_index + 1);
        }
        char_index_map.insert(c, i);
        max_length = max_length.max(i - start + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }
}
