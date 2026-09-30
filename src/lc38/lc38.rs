pub fn count_and_say(n: i32) -> String {
    let mut s = String::from("1");
    for _ in 1..n {
        let bytes = s.as_bytes();
        let mut res = String::new();
        let mut i = 0;
        while i < bytes.len() {
            let mut j = i;
            while j < bytes.len() && bytes[j] == bytes[i] {
                j += 1;
            }
            res.push_str(&(j - i).to_string());
            res.push(bytes[i] as char);
            i = j;
        }
        s = res;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        let s = count_and_say(1);
        assert_eq!(s, "1");
        let s = count_and_say(2);
        assert_eq!(s, "11");
        let s = count_and_say(3);
        assert_eq!(s, "21");
    }
}
