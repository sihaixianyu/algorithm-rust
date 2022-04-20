use std::{cell::RefCell, cmp, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn longest(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        match root {
            Some(root) => {
                let l_len = longest(&root.borrow().left, res);
                let r_len = longest(&root.borrow().right, res);
                *res = cmp::max(*res, l_len + r_len);

                cmp::max(l_len, r_len) + 1
            }
            None => 0,
        }
    }

    let mut res = 0;
    longest(&root, &mut res);

    res
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::BinaryTree;

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from_slice(&[1, 2, 3, 4, 5]);

        let res = diameter_of_binary_tree(tree.root);
        assert_eq!(3, res);
    }

    #[test]
    fn test_case2() {
        let tree = BinaryTree::from_slice(&[1, 2]);

        let res = diameter_of_binary_tree(tree.root);
        assert_eq!(1, res);
    }
}
