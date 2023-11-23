/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            sub_node: &Option<Rc<RefCell<TreeNode>>>,
            comparing: bool,
        ) -> bool {
            match (node, sub_node) {
                (Some(n), Some(sn)) => {
                    let nb = n.borrow();
                    let snb = sn.borrow();
                    let result = if nb.val == snb.val {
                        dfs(&nb.left, &snb.left, true) && dfs(&nb.right, &snb.right, true)
                    } else {
                        false
                    };
                    if comparing {
                        result
                    } else {
                        result || dfs(&nb.left, sub_node, false) || dfs(&nb.right, sub_node, false)
                    }
                }
                (None, None) => true,
                _ => false,
            }
        }
        dfs(&root, &sub_root, false)
    }
}
// @lc code=end
