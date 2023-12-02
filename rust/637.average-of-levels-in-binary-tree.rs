/*
 * @lc app=leetcode id=637 lang=rust
 *
 * [637] Average of Levels in Binary Tree
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut average = Vec::new();
        if let Some(n) = root {
            queue.push_back(n);
            while !queue.is_empty() {
                let mut sum = 0_f64;
                let l = queue.len();
                for _ in 0..l {
                    if let Some(node) = queue.pop_front() {
                        let mut nb = node.borrow_mut();
                        sum += nb.val as f64;
                        if let Some(ln) = nb.left.take() {
                            queue.push_back(ln);
                        }
                        if let Some(rn) = nb.right.take() {
                            queue.push_back(rn);
                        }
                    }
                }
                average.push(sum / l as f64);
            }
        }
        average
    }
}
// @lc code=end
