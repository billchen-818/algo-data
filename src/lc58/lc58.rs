pub fn length_of_last_word(s: String) -> i32 {
    let trimmed = s.trim();
    if let Some(last_word) = trimmed.split_whitespace().last() {
        last_word.len() as i32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".into()), 5);
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".into()), 4);
        assert_eq!(length_of_last_word("luffy is still joyboy".into()), 6);
    }
}
