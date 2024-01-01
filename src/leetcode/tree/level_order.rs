use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::structure::binary_tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut ans = vec![];
    let mut queue = VecDeque::from([root.unwrap()]); // Has been checked at line 7.

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

        ans.push(temp);
    }

    ans
}

#[cfg(test)]
mod tests {

    use crate::structure::binary_tree::{null, BinaryTree};

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([3, 9, 20, null, null, 15, 7].as_slice());

        let ans = level_order(tree.root);
        assert_eq!(ans, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
