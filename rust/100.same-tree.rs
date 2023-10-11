/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => return true,
            (Some(pn), Some(qn)) => {
                let (pb, qb) = (pn.borrow(), qn.borrow());
                let (pl, pr, ql, qr) = (
                    pb.left.clone(),
                    pb.right.clone(),
                    qb.left.clone(),
                    qb.right.clone(),
                );
                return pb.val == qb.val
                    && Self::is_same_tree(pl, ql)
                    && Self::is_same_tree(pr, qr);
            }
            _ => return false,
        }
    }
}
// @lc code=end
