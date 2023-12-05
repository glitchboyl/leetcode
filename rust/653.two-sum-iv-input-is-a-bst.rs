/*
 * @lc app=leetcode id=653 lang=rust
 *
 * [653] Two Sum IV - Input is a BST
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut result = false;
        fn inorder_traverse(
            node: &Option<Rc<RefCell<TreeNode>>>,
            k: &i32,
            set: &mut HashSet<i32>,
            result: &mut bool,
        ) {
            if *result {
                return;
            }
            if let Some(n) = node {
                let nb = n.borrow();
                if set.contains(&(k - nb.val)) {
                    *result = true;
                    return;
                }
                set.insert(nb.val);
                inorder_traverse(&nb.left, k, set, result);
                inorder_traverse(&nb.right, k, set, result);
            }
        }
        inorder_traverse(&root, &k, &mut set, &mut result);
        result
    }
}
// @lc code=end
