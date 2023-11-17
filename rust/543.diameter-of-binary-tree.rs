/*
 * @lc app=leetcode id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut length = 0;
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, length: &mut i32) -> i32 {
            if let Some(n) = node {
                let nb = n.borrow();
                let left_length = dfs(&nb.left, length);
                let right_length = dfs(&nb.right, length);
                *length = i32::max(*length, left_length + right_length);
                return 1 + left_length.max(right_length);
            }
            0
        }
        dfs(&root, &mut length);
        length
    }
}
// @lc code=end
