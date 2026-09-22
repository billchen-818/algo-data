pub fn hamming_weight(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;

    while n != 0 {
        n &= n - 1;
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        let n = 11;
        let target = 3;
        let ret = hamming_weight(n);
        assert_eq!(target, ret);
    }
}
