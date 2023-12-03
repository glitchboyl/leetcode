/*
 * @lc app=leetcode id=606 lang=rust
 *
 * [606] Construct String from Binary Tree
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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut str = String::new();
        if let Some(node) = root {
            let mut nb = node.borrow_mut();
            str.push_str(&nb.val.to_string());
            if nb.left.is_some() || nb.right.is_some() {
                str.push('(');
                str.push_str(&Self::tree2str(nb.left.take()));
                str.push(')');
                if nb.right.is_some() {
                    str.push('(');
                    str.push_str(&Self::tree2str(nb.right.take()));
                    str.push(')');
                }
            }
        }
        str
    }
}
// @lc code=end
