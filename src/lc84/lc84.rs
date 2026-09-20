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

// TODO 单调栈
pub fn largest_rectangle_area_l2(heights: Vec<i32>) -> i32 {
    let mut heights = heights;
    heights.push(0);

    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;

    for i in 0..heights.len() {
        while let Some(&top) = stack.last() {
            if heights[top] > heights[i] {
                // 当前要弹出的柱子高度
                let height = heights[top];

                // 弹出
                stack.pop();

                // 计算左边界
                let left = match stack.last() {
                    Some(&index) => index,
                    None => 0,
                };

                // 计算宽度
                let width = i - left - 1;

                // 面积
                let area = height * width as i32;

                max_area = max_area.max(area);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
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

    #[test]
    fn test_largest_rectangle_area_l2() {
        let l1 = vec![2, 1, 5, 6, 2, 3];
        let ret1 = largest_rectangle_area_l2(l1);
        let target1 = 10;
        assert_eq!(ret1, target1);

        let l1 = vec![2, 4];
        let ret1 = largest_rectangle_area_l2(l1);
        let target1 = 4;
        assert_eq!(ret1, target1);
    }
}
