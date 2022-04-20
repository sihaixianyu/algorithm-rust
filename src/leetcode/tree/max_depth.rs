use std::{cell::RefCell, cmp, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rc) = root {
            let left = &rc.borrow().left;
            let right = &rc.borrow().right;
            return cmp::max(helper(&left) + 1, helper(&right) + 1);
        }

        return 0;
    }

    helper(&root)
}

// pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32, depth: &mut i32) {
//         match root {
//             Some(rc) => {
//                 *depth += 1;
//                 traverse(&rc.borrow().left, res, depth);
//                 traverse(&rc.borrow().right, res, depth);
//                 *depth -= 1;
//             }
//             None => {
//                 if depth > res {
//                     *res = cmp::max(*res, *depth);
//                 }
//                 return;
//             }
//         }
//     }

//     let mut res = 0;
//     let mut depth = 0;
//     traverse(&root, &mut res, &mut depth);

//     res
// }

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from_slice(&[3, 9, 20, null, null, 15, 7]);

        let res = max_depth(tree.root);
        assert_eq!(3, res);
    }

    #[test]
    fn test_case2() {
        let tree = BinaryTree::from_slice(&[1, null, 2]);

        let res = max_depth(tree.root);
        assert_eq!(2, res);
    }
}
