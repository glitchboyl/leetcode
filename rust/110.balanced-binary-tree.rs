/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut max_balance = 0;
        fn get_balance(max_balance: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let nb = node.borrow();
                let left_balance = get_balance(max_balance, nb.left.clone());
                let right_balance = get_balance(max_balance, nb.right.clone());
                *max_balance = *max_balance.max(&mut (left_balance - right_balance).abs());
                return left_balance.max(right_balance) + 1;
            }
            return -1;
        }
        get_balance(&mut max_balance, root);

        return max_balance < 2;
    }
}
// @lc code=end
