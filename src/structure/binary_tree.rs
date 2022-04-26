use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(non_upper_case_globals)]
pub const null: i32 = i32::MAX;

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

    pub fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

pub struct BinaryTree {
    pub root: Link,
    size: usize,
}

impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree {
            root: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn from_slice(vals: &[i32]) -> BinaryTree {
        if vals.len() == 0 {
            return BinaryTree {
                root: None,
                size: 0,
            };
        }

        // Used for skip node of null value
        let mut skip = 0;
        let root = TreeNode::new(vals[0]).into_rc();
        let mut queue = VecDeque::from(vec![root.clone()]);

        let mut i = 1;
        while i < vals.len() {
            if vals[i] == null {
                skip += 1;
                i += 1;
                continue;
            }

            let curr = queue.front().unwrap().clone();
            let new_link = TreeNode::new(vals[i]).into_rc();

            if curr.borrow().left.is_none() {
                if skip == 0 {
                    queue.push_back(new_link.clone());
                    curr.borrow_mut().left = Some(new_link);
                    i += 1;
                    continue;
                } else {
                    skip -= 1;
                }
            }

            if skip == 0 {
                queue.push_back(new_link.clone());
                curr.borrow_mut().right = Some(new_link);
                queue.pop_front();
                i += 1;
                continue;
            } else {
                skip -= 1;
            }

            queue.pop_front();
        }

        BinaryTree {
            root: Some(root),
            size: vals.iter().filter(|&&x| x != null).count(),
        }
    }

    pub fn level_traverse(&self) -> Vec<i32> {
        if self.root.is_none() {
            return vec![];
        }

        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap().clone());

        while queue.len() > 0 {
            let mut curr: Rc<RefCell<TreeNode>>;
            let size = queue.len();

            for _ in 0..size {
                curr = queue.pop_front().unwrap();
                res.push(curr.borrow().val);

                if let Some(left) = &curr.borrow().left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &curr.borrow().right {
                    queue.push_back(right.clone());
                }
            }
        }

        res
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
        let tree = BinaryTree::from_slice(&vec![1, 2, 3, null, null, 4, 5][..]);
        let res = tree.level_traverse();
        assert_eq!(vec![1, 2, 3, 4, 5], res)
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
