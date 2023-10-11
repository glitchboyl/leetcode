/*
 * @lc app=leetcode id=257 lang=rust
 *
 * [257] Binary Tree Paths
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, path: &str) {
            if let Some(n) = node {
                let nb = n.borrow();
                let mut new_path = format!("{path}{}", nb.val);
                if nb.left.is_none() && nb.right.is_none() {
                    (*paths).push(new_path);
                } else {
                    new_path.push_str("->");
                    dfs(nb.left.clone(), paths, &new_path);
                    dfs(nb.right.clone(), paths, &new_path);
                }
            }
        }
        dfs(root, &mut paths, "");
        return paths;
    }
}
// @lc code=end
