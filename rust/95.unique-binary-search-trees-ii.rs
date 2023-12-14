/*
 * @lc app=leetcode id=95 lang=rust
 *
 * [95] Unique Binary Search Trees II
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as usize;
        let mut cache = vec![vec![Vec::new(); n + 1]; n];
        fn generate_trees_from(
            start: usize,
            end: usize,
            cache: &mut Vec<Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>>,
        ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if start > end {
                return vec![None];
            }
            if cache[start - 1][end].is_empty() {
                for i in start..=end {
                    let left_trees = generate_trees_from(start, i - 1, cache);
                    let right_trees = generate_trees_from(i + 1, end, cache);
                    for left_tree in &left_trees {
                        for right_tree in &right_trees {
                            cache[start - 1][end].push(Some(Rc::new(RefCell::new(TreeNode {
                                val: i as i32,
                                left: left_tree.clone(),
                                right: right_tree.clone(),
                            }))));
                        }
                    }
                }
            }
            cache[start - 1][end].to_vec()
        }
        generate_trees_from(1, n, &mut cache)
    }
}
// @lc code=end
