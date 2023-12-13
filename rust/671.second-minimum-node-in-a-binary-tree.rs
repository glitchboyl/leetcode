/*
 * @lc app=leetcode id=671 lang=rust
 *
 * [671] Second Minimum Node In a Binary Tree
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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut second_minimum = i32::MAX;
        let mut exists = false;
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, second_minimum: &mut i32, exists: &mut bool) {
            if let Some(n) = node {
                let nb = n.borrow();
                if nb.left.is_some() {
                    let mut left_val = nb.left.as_ref().unwrap().borrow().val;
                    if nb.val == left_val {
                        dfs(&nb.left, second_minimum, exists);
                    } else {
                        *second_minimum = *second_minimum.min(&mut left_val);
                        *exists = true;
                    }
                }
                if nb.right.is_some() {
                    let mut right_val = nb.right.as_ref().unwrap().borrow().val;
                    if nb.val == right_val {
                        dfs(&nb.right, second_minimum, exists);
                    } else {
                        *second_minimum = *second_minimum.min(&mut right_val);
                        *exists = true;
                    }
                }
            }
        }
        dfs(&root, &mut second_minimum, &mut exists);
        if !exists {
            return -1;
        }
        second_minimum
    }
}
// @lc code=end
