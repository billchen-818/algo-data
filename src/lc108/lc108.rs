// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
    let n = nums.len();

    match n {
        0 => None,
        _ => {
            let m = n / 2;
            let mut node = TreeNode::new(nums[m]);
            node.left = sorted_array_to_bst(nums[..m].to_vec());
            node.right = sorted_array_to_bst(nums[m + 1..].to_vec());

            Some(Rc::new(RefCell::new(node)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst() {
        let v = vec![-10, -3, 0, 5, 9];
        let _ = sorted_array_to_bst(v);
    }
}
