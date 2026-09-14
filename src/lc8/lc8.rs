pub fn my_atoi(s: String) -> i32 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }

    let mut chars = s.chars();
    let mut sign = 1;
    let mut result: i32 = 0;

    if let Some(first_char) = chars.next() {
        match first_char {
            '-' => sign = -1,
            '+' => sign = 1,
            c if c.is_digit(10) => result = c.to_digit(10).unwrap() as i32,
            _ => return 0,
        }
    }

    for c in chars {
        if !c.is_digit(10) {
            break;
        }
        let digit = c.to_digit(10).unwrap() as i32;

        // Check for overflow
        if result > (i32::MAX - digit) / 10 {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }

        result = result * 10 + digit;
    }

    result * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(my_atoi("42".into()), 42);
        assert_eq!(my_atoi("   -42".into()), -42);
        assert_eq!(my_atoi("4193 with words".into()), 4193);
    }
}
