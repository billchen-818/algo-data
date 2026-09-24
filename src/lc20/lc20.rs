use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut hm: HashMap<char, char> = HashMap::new();

    hm.insert('(', ')');
    hm.insert('[', ']');
    hm.insert('{', '}');

    let mut stack: Vec<char> = vec![];

    for a in s.chars() {
        if hm.contains_key(&a) {
            stack.push(a);
        } else {
            let v = stack.pop();

            match v {
                Some(open) if hm.get(&open) == Some(&a) => {}
                _ => return false,
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s = "()".to_string();
        let res = is_valid(s);
        assert!(res);

        let s = "([)]".to_string();
        let res = is_valid(s);
        assert!(!res);

        let s = "([])".to_string();
        let res = is_valid(s);
        assert!(res);

        let s = "]".to_string();
        let res = is_valid(s);
        assert!(!res);
    }
}
