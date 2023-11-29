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
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, str: &mut String) {
            if let Some(n) = node {
                let nb = n.borrow();
                str.push_str(&nb.val.to_string());
                if nb.left.is_some() {
                    str.push('(');
                    dfs(&nb.left, str);
                    str.push(')');
                }
                if nb.right.is_some() {
                    if nb.left.is_none() {
                        str.push('(');
                        str.push(')');
                    }
                    str.push('(');
                    dfs(&nb.right, str);
                    str.push(')');
                }
            }
        }
        dfs(&root, &mut str);
        str
    }
}
// @lc code=end
