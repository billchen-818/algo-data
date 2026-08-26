pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;

    // 从最右侧开始比较，把最大值放到nums1最右侧
    for k in (0..(m + n)).rev() {
        if i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
        } else if j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
    }
}
