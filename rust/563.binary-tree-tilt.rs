/*
 * @lc app=leetcode id=563 lang=rust
 *
 * [563] Binary Tree Tilt
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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tilt = 0;
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, tilt: &mut i32) -> i32 {
            if let Some(n) = node {
                let nb = n.borrow();
                let l = traverse(&nb.left, tilt);
                let r = traverse(&nb.right, tilt);
                *tilt += (l - r).abs();
                return l + r + nb.val;
            }
            0
        }
        traverse(&root, &mut tilt);
        tilt
    }
}
// @lc code=end
