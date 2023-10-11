/*
 * @lc app=leetcode id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let nb = node.borrow();
            let (mut lh, mut rh) = (0, 0);
            let (mut left, mut right) = (nb.left.clone(), nb.right.clone());
            while let Some(ln) = left {
                lh += 1;
                left = ln.borrow().left.clone();
            }
            while let Some(rn) = right {
                rh += 1;
                right = rn.borrow().right.clone();
            }
            // it is full binary tree
            if lh == rh {
                return 2_i32.pow(lh + 1) - 1;
            }
            return 1 + Self::count_nodes(nb.left.clone()) + Self::count_nodes(nb.right.clone());
        }
        return 0;
    }
}
// @lc code=end
