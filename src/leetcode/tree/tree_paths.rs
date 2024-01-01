use std::{cell::RefCell, rc::Rc};

use crate::structure::binary_tree::TreeNode;

// 二叉树的所有路径：https://leetcode.cn/problems/binary-tree-paths/
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn dfs(root: Rc<RefCell<TreeNode>>, mut path: String, ans: &mut Vec<String>) {
        path.push_str(&root.borrow().val.to_string());

        let left = &root.borrow().left;
        let right = &root.borrow().right;

        if left.is_none() && right.is_none() {
            ans.push(path.to_string());
        }

        path.push_str("->");

        if let Some(left) = left {
            dfs(left.clone(), path.clone(), ans);
        }
        if let Some(right) = right {
            dfs(right.clone(), path.clone(), ans);
        }

        path.push_str(&root.borrow().val.to_string());
    }

    let mut ans = vec![];
    let path = "".to_string();

    if let Some(root) = root {
        dfs(root.clone(), path, &mut ans);
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::structure::binary_tree::{null, BinaryTree};

    use super::*;

    #[test]
    fn test_case1() {
        let tree = BinaryTree::from([1, 2, 3, null, 5].as_slice());

        let ans = binary_tree_paths(tree.root);
        assert_eq!(ans, ["1->2->5".to_string(), "1->3".to_string()]);
    }
}
