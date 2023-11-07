/*
 * @lc app=leetcode id=501 lang=rust
 *
 * [501] Find Mode in Binary Search Tree
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut modes = Vec::new();
        fn inorder_traversal(
            node: &Option<Rc<RefCell<TreeNode>>>,
            modes: &mut Vec<i32>,
            current: &mut (i32, i32),
            max_times: &mut i32,
        ) {
            if let Some(n) = node {
                let nb = n.borrow();
                inorder_traversal(&nb.left, modes, current, max_times);
                *current = (
                    nb.val,
                    if current.0 == nb.val {
                        current.1 + 1
                    } else {
                        1
                    },
                );
                if current.1 >= *max_times {
                    if current.1 > *max_times {
                        *max_times = current.1;
                        modes.clear();
                    }
                    modes.push(current.0);
                }
                inorder_traversal(&nb.right, modes, current, max_times);
            }
        }
        inorder_traversal(&root, &mut modes, &mut (0, 0), &mut 0);
        modes
    }
}
// @lc code=end
