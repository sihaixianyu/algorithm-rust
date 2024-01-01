use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

// 对称二叉树：https://leetcode.cn/problems/symmetric-tree/
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                return left.borrow().val == right.borrow().val
                    && helper(&left.borrow().left, &right.borrow().right)
                    && helper(&left.borrow().right, &right.borrow().left)
            }
            _ => false,
        }
    }

    let root = root.as_ref().unwrap();

    helper(&root.borrow().left, &root.borrow().right)
}

pub fn is_symmetric2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() || right.is_none() {
            return false;
        }

        let left = left.as_ref().unwrap();
        let right = right.as_ref().unwrap();

        if left.borrow().val != right.borrow().val {
            return false;
        }

        helper(&left.borrow().left, &right.borrow().right)
            && helper(&left.borrow().right, &right.borrow().left)
    }

    let root = root.as_ref().unwrap();

    helper(&root.borrow().left, &root.borrow().right)
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::BinaryTree;

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([1, 2, 2, 3, 4, 4, 3].as_slice());

        let ans = is_symmetric(tree.root);
        assert_eq!(ans, true);
    }

    #[test]
    fn test_case2() {
        let tree = BinaryTree::from([1, 2, 2, 3, 4, 4, 3].as_slice());

        let ans = is_symmetric2(tree.root);
        assert_eq!(ans, true);
    }
}
