use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

// 判断是否为平衡二叉树：https://leetcode.cn/problems/balanced-binary-tree/
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    get_depth(&root) != -1
}

fn get_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_depth = get_depth(&root.as_ref().unwrap().borrow().left);
    let right_depth = get_depth(&root.as_ref().unwrap().borrow().right);

    if left_depth == -1 || right_depth == -1 {
        return -1;
    }

    if (left_depth - right_depth).abs() <= 1 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([3, 9, 20, null, null, 15, 7].as_slice());

        let ans = is_balanced(tree.root);
        assert_eq!(true, ans);
    }
}
