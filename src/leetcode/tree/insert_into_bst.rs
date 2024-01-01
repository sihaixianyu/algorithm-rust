use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let new_node = Rc::new(RefCell::new(TreeNode::new(val)));

    if root.is_none() {
        return Some(new_node);
    }

    let mut cur = root.clone();
    let mut parent = None;

    while let Some(curr_rc) = cur.clone() {
        parent = cur.clone();
        if val < curr_rc.borrow().val {
            cur = curr_rc.borrow().left.clone();
        } else {
            cur = curr_rc.borrow().right.clone();
        }
    }

    if let Some(parent) = parent {
        if val < parent.borrow().val {
            parent.borrow_mut().left = Some(new_node);
        } else {
            parent.borrow_mut().right = Some(new_node);
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::insert_into_bst;

    #[test]
    fn test_case_1() {
        let root = BinaryTree::from([40, 20, 60, 10, 30, 50, 70].as_slice()).root;
        let val = 25;

        let real = insert_into_bst(root, val);
        let expected =
            BinaryTree::from([40, 20, 60, 10, 30, 50, 70, null, null, 25].as_slice()).root;

        assert_eq!(real, expected);
    }

    #[test]
    fn test_case_2() {
        let root = BinaryTree::from([4, 2, 7, 1, 3].as_slice()).root;
        let val = 5;

        let real = insert_into_bst(root, val);
        let expected = BinaryTree::from([4, 2, 7, 1, 3, 5].as_slice()).root;

        assert_eq!(real, expected);
    }
}
