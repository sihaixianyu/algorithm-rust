use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub type Link = Option<Rc<RefCell<TreeNode>>>;

#[allow(non_upper_case_globals)]
pub const null: i32 = i32::MAX;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Link,
    pub right: Link,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Default)]
pub struct BinaryTree {
    pub root: Link,
}

// TEST: New version of `From<&[i32]>` trait impl for creating a binary tree by level.
impl From<&[i32]> for BinaryTree {
    fn from(nums: &[i32]) -> Self {
        if nums.len() == 0 {
            return BinaryTree::default();
        }

        let mut nodes = VecDeque::new();

        for &num in nums.iter() {
            if num == null {
                nodes.push_back(None);
                continue;
            }

            let node = Rc::new(RefCell::new(TreeNode::new(num)));
            nodes.push_back(Some(node));
        }

        for i in 0..(nums.len() / 2) {
            let Some(node) = nodes[i].clone() else {
                continue;
            };

            if 2 * i + 1 < nums.len() {
                node.borrow_mut().left = nodes[2 * i + 1].clone();
            }
            if 2 * i + 2 < nums.len() {
                node.borrow_mut().right = nodes[2 * i + 2].clone();
            }
        }

        BinaryTree {
            root: nodes[0].clone(),
        }
    }
}

impl BinaryTree {
    pub fn travese_level(&self) -> Vec<i32> {
        return level_traverse(&self.root);
    }

    pub fn travese_preorder(&self) -> Vec<i32> {
        return preorder_traverse(&self.root);
    }

    pub fn travese_inorder(&self) -> Vec<i32> {
        return inorder_traverse(&self.root);
    }

    pub fn travese_postorder(&self) -> Vec<i32> {
        return postorder_traverse(&self.root);
    }
}

// Level travese binary tree through root.
pub fn level_traverse(root: &Link) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut ans = vec![];
    let mut queue = VecDeque::from([root.as_ref().unwrap().clone()]);

    while let Some(cur) = queue.front().cloned() {
        ans.push(cur.borrow().val);

        if let Some(left) = cur.borrow().left.clone() {
            queue.push_back(left);
        }
        if let Some(right) = cur.borrow().right.clone() {
            queue.push_back(right)
        }

        queue.pop_front();
    }

    ans
}

// Level travese binary tree through root.
pub fn preorder_traverse(root: &Link) -> Vec<i32> {
    fn helper(root: &Link, ans: &mut Vec<i32>) {
        let Some(node) = root else {
            return;
        };

        ans.push(node.borrow().val);
        helper(&node.borrow().left, ans);
        helper(&node.borrow().right, ans);
    }

    let mut ans = vec![];
    helper(root, &mut ans);

    ans
}

// Level travese binary tree through root.
pub fn inorder_traverse(root: &Link) -> Vec<i32> {
    fn helper(root: &Link, ans: &mut Vec<i32>) {
        let Some(node) = root else {
            return;
        };

        helper(&node.borrow().left, ans);
        ans.push(node.borrow().val);
        helper(&node.borrow().right, ans);
    }

    let mut ans = vec![];
    helper(&root, &mut ans);

    ans
}

// Level travese binary tree through root.
pub fn postorder_traverse(root: &Link) -> Vec<i32> {
    fn helper(root: &Link, ans: &mut Vec<i32>) {
        let Some(node) = root else {
            return;
        };

        helper(&node.borrow().left, ans);
        helper(&node.borrow().right, ans);
        ans.push(node.borrow().val);
    }

    let mut ans = vec![];
    helper(&root, &mut ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let nums = [1, 2, 3, null, 4, null, 5];
        let tree = BinaryTree::from(nums.as_slice());

        let ans = tree.travese_preorder();
        assert_eq!(ans, vec![1, 2, 4, 3, 5]);

        let ans = tree.travese_inorder();
        assert_eq!(ans, vec![2, 4, 1, 3, 5]);

        let ans = tree.travese_postorder();
        assert_eq!(ans, vec![4, 2, 5, 3, 1]);
    }

    #[test]
    fn test_from_slice_2() {
        let _tree = BinaryTree::from([2, null, 3, null, 4, null, 5, null, 6].as_slice());
    }

    #[test]
    fn test_levelorder_traverse() {
        let nums = [1, 2, 3, null, 4, null, 5];
        let tree = BinaryTree::from(nums.as_slice());

        let ans = tree.travese_level();
        assert_eq!(ans, vec![1, 2, 3, 4, 5]);
    }
}
