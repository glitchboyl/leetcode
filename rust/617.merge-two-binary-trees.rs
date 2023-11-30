/*
 * @lc app=leetcode id=617 lang=rust
 *
 * [617] Merge Two Binary Trees
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let (mut nb1, mut nb2) = (node1.borrow_mut(), node2.borrow_mut());
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nb1.val + nb2.val,
                    left: Self::merge_trees(nb1.left.take(), nb2.left.take()),
                    right: Self::merge_trees(nb1.right.take(), nb2.right.take()),
                })))
            }
            (Some(node), None) | (None, Some(node)) => Some(node),
            _ => None,
        }
    }
}
// @lc code=end
