/*
 * @lc app=leetcode id=99 lang=rust
 *
 * [99] Recover Binary Search Tree
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut mistake_nodes = (None, None);
        let mut pre_node = None;
        fn inorder_traverse(
            node: &Option<Rc<RefCell<TreeNode>>>,
            mistake_nodes: &mut (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>),
            pre_node: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = &node {
                let mut nb = n.borrow();
                inorder_traverse(&nb.left, mistake_nodes, pre_node);
                if let Some(pn) = pre_node {
                    if nb.val < pn.borrow().val {
                        if mistake_nodes.0.is_none() {
                            *mistake_nodes = (Some(Rc::clone(pn)), Some(Rc::clone(n)));
                        } else {
                            mistake_nodes.1 = Some(Rc::clone(n));
                        }
                    }
                }
                *pre_node = Some(Rc::clone(n));
                inorder_traverse(&nb.right, mistake_nodes, pre_node);
            }
        }
        inorder_traverse(root, &mut mistake_nodes, &mut pre_node);
        if let (Some(first), Some(second)) = mistake_nodes {
            std::mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val);
        }
    }
}
// @lc code=end
