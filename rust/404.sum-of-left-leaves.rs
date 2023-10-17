/*
 * @lc app=leetcode id=404 lang=rust
 *
 * [404] Sum of Left Leaves
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let nb = node.borrow();
            let left_node = &nb.left;
            if let Some(ln) = left_node {
                let lnb = ln.borrow();
                if lnb.left.is_none() && lnb.right.is_none() {
                    sum += lnb.val;
                } else {
                    sum += Self::sum_of_left_leaves(left_node.clone());
                }
            }
            sum += Self::sum_of_left_leaves(nb.right.clone());
        }
        return sum;
    }
}
// @lc code=end
