use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let root_rc = root.as_ref().unwrap();

    let left = lowest_common_ancestor(root_rc.borrow().left.clone(), p.clone(), q.clone());
    let right = lowest_common_ancestor(root_rc.borrow().right.clone(), p.clone(), q.clone());

    match (left, right) {
        (Some(left), None) if root != p && root != q => Some(left),
        (None, Some(right)) if root != p && root != q => Some(right),
        (None, None) if root != p && root != q => None,
        _ => root,
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::lowest_common_ancestor;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5].as_slice());
        let root = tree.root.expect("root is none!");
        let p = root.borrow().left.clone();
        let q = root.borrow().right.clone();

        let ans = lowest_common_ancestor(Some(root.clone()), p, q);
        assert_eq!(ans, Some(root));
    }

    #[test]
    fn test_case2() {
        let tree = BinaryTree::from([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5].as_slice());

        let root = tree.root.clone();
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = p.as_ref().unwrap().borrow().right.clone();

        let ans = lowest_common_ancestor(root, p.clone(), q);
        assert_eq!(ans, p);
    }
}
