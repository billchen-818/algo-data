pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;

    // 从后往前遍历
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits; // 没有进位，直接返回
        }
        digits[i] = 0; // 当前位是 9，进位置 0，继续往前
    }

    // 循环结束说明全是 9，比如 [9,9,9] -> 需要在最前面插入 1
    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let dig1 = vec![1, 2, 3];
        let taget1 = vec![1, 2, 4];
        let res1 = plus_one(dig1);
        assert_eq!(res1, taget1);

        let dig2 = vec![9];
        let target2 = vec![1, 0];
        let res2 = plus_one(dig2);
        assert_eq!(res2, target2);
    }
}
