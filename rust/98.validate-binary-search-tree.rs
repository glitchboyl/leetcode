/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut current = None;
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current: &mut Option<i32>) -> bool {
            if let Some(n) = node {
                let nb = n.borrow();
                let mut result = dfs(&nb.left, current);
                if result {
                    if let Some(v) = &current {
                        result = nb.val > *v;
                    }
                    if result {
                        *current = Some(nb.val);
                        result = dfs(&nb.right, current);
                    }
                }
                return result;
            }
            true
        }
        dfs(&root, &mut current)
    }
}
// @lc code=end
