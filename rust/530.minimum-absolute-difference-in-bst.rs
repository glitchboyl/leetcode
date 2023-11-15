/*
 * @lc app=leetcode id=530 lang=rust
 *
 * [530] Minimum Absolute Difference in BST
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = i32::MAX;
        fn inorder_traversal(
            node: &Option<Rc<RefCell<TreeNode>>>,
            min_diff: &mut i32,
            prev: &mut Option<i32>,
        ) {
            if let Some(n) = node {
                let nb = n.borrow();
                inorder_traversal(&nb.left, min_diff, prev);
                if prev.is_some() {
                    *min_diff = *min_diff.min(&mut (nb.val - prev.unwrap()));
                }
                *prev = Some(nb.val);
                inorder_traversal(&nb.right, min_diff, prev);
            }
        }
        inorder_traversal(&root, &mut min_diff, &mut None);
        min_diff
    }
}
// @lc code=end
