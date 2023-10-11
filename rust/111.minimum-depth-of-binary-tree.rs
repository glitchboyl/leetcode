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
        if let Some(node) = root {
            // Solution 1: BFS
            let mut d = 0;
            let mut nodes = vec![vec![node]];
            while let Some(level) = nodes.pop() {
                d += 1;
                let mut children_nodes = vec![];
                for n in level {
                    let nb = n.borrow();
                    match (nb.left.clone(), nb.right.clone()) {
                        (None, None) => return d,
                        (Some(cn), None) | (None, Some(cn)) => {
                            children_nodes.push(cn);
                        }
                        (Some(ln), Some(rn)) => {
                            children_nodes.push(ln);
                            children_nodes.push(rn);
                        }
                    }
                }
                if children_nodes.len() > 0 {
                    nodes.push(children_nodes)
                }
            }

            // Solution 2: DFS
            // let nb = node.borrow();
            // return 1 + match (nb.left.clone(), nb.right.clone()) {
            //     (None, None) => 0,
            //     (Some(cn), None) | (None, Some(cn)) => Self::min_depth(Some(cn)),
            //     (Some(ln), Some(rn)) => {
            //         i32::min(Self::min_depth(Some(ln)), Self::min_depth(Some(rn)))
            //     }
            // };
        }
        return 0;
    }
}
// @lc code=end
