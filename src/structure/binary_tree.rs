use std::{cell::RefCell, rc::Rc, collections::VecDeque};

type Link = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Link,
    pub right: Link,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn as_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

#[allow(non_upper_case_globals)]
const null: i32 = i32::MAX;

pub struct BinaryTree {
    root: Link,
    len: usize,
}

impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn from_slice(vals: &[i32]) -> BinaryTree {
        if vals.len() == 0 {
            return BinaryTree { root: None, len: 0 };
        }

        let root = TreeNode::new(vals[0]).as_rc();
        let mut queue = VecDeque::from(vec![root.clone()]);
        // Used for skip node of null value
        let mut skip = 0;

        let mut i = 1;
        while i < vals.len() {
            if vals[i] == null {
                skip += 1;
            }

            let curr = queue.front().unwrap().clone();
            let new_link = TreeNode::new(vals[i]).as_rc();

            if curr.borrow().left.is_none() {
                if skip > 0 {
                    skip -= 1;
                } else {
                    queue.push_back(new_link.clone());
                    curr.borrow_mut().left = Some(new_link);

                    i += 1;
                    continue;
                }
            }
            if curr.borrow().right.is_none() {
                if skip > 0 {
                    skip -= 1;
                } else {
                    queue.pop_front();
                    queue.push_back(new_link.clone());
                    curr.borrow_mut().right = Some(new_link);

                    i += 1;
                    continue;
                }
            }

            queue.pop_front();

            if skip == 0 {
                i += 1;
            }
        }

        BinaryTree {
            root: Some(root),
            len: vals.len(),
        }
    }
}

pub fn preorder_traverse(root: &Link) -> Vec<i32> {
    let mut res = vec![];

    fn helper(root: &Link, res: &mut Vec<i32>) {
        if let Some(rc) = root {
            res.push(rc.borrow().val);
            helper(&rc.borrow().left, res);
            helper(&rc.borrow().right, res);
        }
    }

    helper(root, &mut res);

    res
}

pub fn inorder_traverse(root: &Link) -> Vec<i32> {
    let mut res = vec![];

    fn helper(root: &Link, res: &mut Vec<i32>) {
        if let Some(rc) = root {
            helper(&rc.borrow().left, res);
            res.push(rc.borrow().val);
            helper(&rc.borrow().right, res);
        }
    }

    helper(root, &mut res);

    res
}

pub fn postorder_traverse(root: &Link) -> Vec<i32> {
    let mut res = vec![];

    fn helper(root: &Link, res: &mut Vec<i32>) {
        if let Some(rc) = root {
            helper(&rc.borrow().left, res);
            helper(&rc.borrow().right, res);
            res.push(rc.borrow().val);
        }
    }

    helper(root, &mut res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3][..]);
        assert_eq!(3, tree.len());
    }
    #[test]
    fn test_preorder_traverse() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3][..]);
        assert_eq!(vec![1, 2, 3], preorder_traverse(&tree.root));
    }

    #[test]
    fn test_inorder_traverse() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3][..]);
        assert_eq!(vec![2, 1, 3], inorder_traverse(&tree.root));
    }

    #[test]
    fn test_postorder_traverse() {
        let tree = BinaryTree::from_slice(&vec![1, 2, 3][..]);
        assert_eq!(vec![2, 3, 1], postorder_traverse(&tree.root));
    }
}
