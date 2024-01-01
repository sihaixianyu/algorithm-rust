use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::structure::binary_tree::TreeNode;

// 二叉树的右视图：https://leetcode.cn/problems/binary-tree-right-side-view/
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut ans = vec![];
    let mut queue = VecDeque::from([root.unwrap()]);

    while queue.len() > 0 {
        let mut temp = vec![];

        for _ in 0..queue.len() {
            let cur = queue.pop_front().unwrap();

            if cur.borrow().left.is_some() {
                queue.push_back(cur.borrow().left.clone().unwrap());
            }
            if cur.borrow().right.is_some() {
                queue.push_back(cur.borrow().right.clone().unwrap());
            }

            temp.push(cur.borrow().val);
        }

        ans.push(*temp.last().unwrap());
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([1, 2, 3, null, 5, null, 4].as_slice());
        let ans = right_side_view(tree.root);

        assert_eq!(ans, [1, 3, 4]);
    }
}
