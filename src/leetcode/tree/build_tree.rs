use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 || postorder.len() == 0 {
            return None;
        }

        println!("inorder: {:?}, postorder: {:?}", inorder, postorder);
        let mut root = TreeNode::new(*postorder.last().unwrap());

        if inorder.len() > 1 && postorder.len() > 1 {
            let root_val = root.val;
            let root_idx = inorder.iter().position(|x| *x == root_val).unwrap();

            root.left = helper(&inorder[..root_idx], &postorder[..root_idx]);
            root.right = helper(
                &inorder[root_idx + 1..],
                &postorder[root_idx..postorder.len() - 1],
            );
        }

        return Some(Rc::new(RefCell::new(root)));
    }

    helper(&inorder[..], &postorder[..])
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::build_tree;

    #[test]
    fn test_case1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];

        let ans = build_tree(inorder, postorder);
        assert_eq!(
            ans,
            BinaryTree::from([3, 9, 20, null, null, 15, 7].as_slice()).root
        )
    }

    #[test]
    fn test_case2() {
        let inorder = vec![2, 1];
        let postorder = vec![2, 1];

        let ans = build_tree(inorder, postorder);
        assert_eq!(ans, BinaryTree::from([1, 2].as_slice()).root)
    }
}
