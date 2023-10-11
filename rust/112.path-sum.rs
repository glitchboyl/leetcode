/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let nb = node.borrow();
            if nb.left.is_none() && nb.right.is_none() && target_sum == nb.val {
                return true;
            }
            return Self::has_path_sum(nb.left.clone(), target_sum - nb.val)
                || Self::has_path_sum(nb.right.clone(), target_sum - nb.val);
        }
        return false;
    }
}
// @lc code=end
