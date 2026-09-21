use std::cell::RefCell;
use std::rc::Rc;

// #[derive(Debug, PartialEq, Eq)]
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

// 中序遍历
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            traversal(n.borrow().left.clone(), res);
            res.push(n.borrow().val);
            traversal(n.borrow().right.clone(), res);
        }
    }

    traversal(root, &mut res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        // root = [1, null, 2, 3]
        // target = [1, 3, 2]

        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().right = Some(node2.clone());
        node2.borrow_mut().left = Some(node3.clone());

        let target = vec![1, 3, 2];
        let res = inorder_traversal(Some(root));

        assert_eq!(res, target);
    }
}
