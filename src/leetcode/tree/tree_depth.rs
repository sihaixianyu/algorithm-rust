use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use crate::structure::binary_tree::TreeNode;

// 二叉树的最大深度：https://leetcode.cn/problems/maximum-depth-of-binary-tree/
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(curr: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if curr.is_none() {
            return 0;
        }

        let left = &curr.as_ref().unwrap().borrow().left;
        let right = &curr.as_ref().unwrap().borrow().right;

        max(helper(left), helper(right)) + 1
    }

    helper(&root)
}

// 二叉树最小深度：https://leetcode.cn/problems/minimum-depth-of-binary-tree/
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(curr: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if curr.is_none() {
            return 0;
        }

        let left = &curr.as_ref().unwrap().borrow().left;
        let right = &curr.as_ref().unwrap().borrow().right;

        dbg!(left, right);
        if left.is_none() && right.is_some() {
            return 1 + helper(right);
        }
        if left.is_some() && right.is_none() {
            return 1 + helper(left);
        }

        min(helper(left), helper(right)) + 1
    }

    helper(&root)
}

#[cfg(test)]
mod tests {
    use crate::{
        leetcode::tree::tree_depth::min_depth,
        structure::binary_tree::{null, BinaryTree},
    };

    use super::max_depth;

    #[test]
    fn test_max_depth() {
        let tree = BinaryTree::from([3, 9, 20, null, null, 15, 7].as_slice());

        let ans = max_depth(tree.root);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_min_depth() {
        let tree = BinaryTree::from([3, 9, 20, null, null, 15, 7].as_slice());

        let ans = min_depth(tree.root);
        assert_eq!(ans, 2);
    }
}
