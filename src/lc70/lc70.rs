pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else {
        1 + climb_stairs(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        let n = 2;
        let res = climb_stairs(n);
        assert_eq!(res, 2);

        let n = 3;
        let res = climb_stairs(n);
        assert_eq!(res, 3);
    }
}
