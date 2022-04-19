use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut height = 1;
    let mut queue = VecDeque::from([root.unwrap()]);

    while queue.len() > 0 {
        let mut curr: Rc<RefCell<TreeNode>>;
        let size = queue.len();

        for _ in 0..size {
            curr = queue.pop_front().unwrap();

            if curr.borrow().left.is_none() && curr.borrow().right.is_none() {
                return height;
            }

            if let Some(left) = &curr.borrow().left {
                queue.push_back(left.clone());
            }

            if let Some(right) = &curr.borrow().right {
                queue.push_back(right.clone());
            }
        }

        height += 1;
    }

    height
}

#[cfg(test)]
pub mod tests {
    use crate::structure::binary_tree::{null, preorder_traverse, BinaryTree};

    use super::*;

    #[test]
    pub fn test_case1() {
        let tree = BinaryTree::from_slice(&vec![3, 9, 20, null, null, 15, 7]);
        let res = min_depth(tree.root);
        assert_eq!(2, res);
    }

    #[test]
    pub fn test_case2() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3, 4, 5]);
        let res = min_depth(tree.root);
        assert_eq!(2, res);
    }

    #[test]
    pub fn test_case3() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3, 4, null, null, 5]);
        println!("{:?}", preorder_traverse(&tree.root));
        let res = min_depth(tree.root);
        assert_eq!(3, res);
    }
}
