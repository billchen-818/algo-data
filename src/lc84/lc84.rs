// 方法一：暴力破解
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max: i32 = 0;

    let length = heights.len();

    println!("length = {}", length);

    for i in 0..length {
        let mut left = i;
        let mut right = i;

        while left > 0 && heights[left - 1] >= heights[i] {
            left -= 1;
        }

        while right < length - 1 && heights[right + 1] >= heights[i] {
            right = right + 1;
        }
        println!(
            "i = {}, heights[{}]={}, left={}, right = {}",
            i, i, heights[i], left, right
        );

        let wid = (right - left + 1) as i32;
        let s = wid * heights[i];
        if s > max {
            max = s;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        let l1 = vec![2, 1, 5, 6, 2, 3];
        let ret1 = largest_rectangle_area(l1);
        let target1 = 10;
        assert_eq!(ret1, target1);

        let l1 = vec![2, 4];
        let ret1 = largest_rectangle_area(l1);
        let target1 = 4;
        assert_eq!(ret1, target1);
    }
}
