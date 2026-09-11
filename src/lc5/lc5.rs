pub fn longest_palindrome(s: String) -> String {
    let mut start = 0;
    let mut end = 0;

    for i in 0..s.len() {
        let len1 = expand_around_center(&s, i, i);
        let len2 = expand_around_center(&s, i, i + 1);
        let len = len1.max(len2);
        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }
    s[start..=end].into()
}

fn expand_around_center(s: &str, left: usize, right: usize) -> usize {
    let mut l = left as isize;
    let mut r = right as isize;
    let chars: Vec<char> = s.chars().collect();

    while l >= 0 && r < chars.len() as isize && chars[l as usize] == chars[r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babcd".into()), "bab");
        assert_eq!(longest_palindrome("cbbd".into()), "bb");
    }
}
