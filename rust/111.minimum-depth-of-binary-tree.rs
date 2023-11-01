/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Solution 1: BFS
        use std::collections::VecDeque;
        let mut nodes = VecDeque::from([(1, root)]);
        while let Some((depth, Some(n))) = nodes.pop_front() {
            let mut nb = n.borrow_mut();
            if nb.left.is_none() && nb.right.is_none() {
                return depth;
            }
            if nb.left.is_some() {
                nodes.push_back((depth + 1, nb.left.take()));
            }
            if nb.right.is_some() {
                nodes.push_back((depth + 1, nb.right.take()));
            }
        }
        0

        // Solution 2: DFS
        // fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //     if let Some(n) = node {
        //         let nb = n.borrow();
        //         1 + match (&nb.left, &nb.right) {
        //             (None, None) => 0,
        //             (Some(_), None) => dfs(&nb.left),
        //             (None, Some(_)) => dfs(&nb.right),
        //             (Some(_), Some(_)) => dfs(&nb.left).min(dfs(&nb.right)),
        //         }
        //     } else {
        //         0
        //     }
        // }
        // dfs(&root)
    }
}
// @lc code=end
