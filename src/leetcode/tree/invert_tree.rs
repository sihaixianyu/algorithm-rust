use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

// 翻转二叉树：https://leetcode.cn/problems/invert-binary-tree/
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let Some(node) = root else {
        return None;
    };

    let left = node.borrow().left.clone();
    let right = node.borrow().right.clone();

    node.borrow_mut().left = invert_tree(right);
    node.borrow_mut().right = invert_tree(left);

    Some(node)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::structure::binary_tree::BinaryTree;

    use super::*;

    #[test]
    fn test_case1() {
        let mut tree = BinaryTree::from([4, 2, 7, 1, 3, 6, 9].as_slice());

        let new_root = invert_tree(tree.root);
        tree.root = new_root;
        let ans = tree.travese_level();
        assert_eq!(ans, vec![4, 7, 2, 9, 6, 3, 1]);
    }
}
