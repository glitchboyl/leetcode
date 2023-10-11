/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result = head.clone();
        let mut current = &mut result;
        // idk why `while let` doesn't work so i checked the solution
        // and modified like this:
        loop {
            match current {
                Some(node) if node.val == val => {
                    *current = node.next.take();
                }
                Some(node) => {
                    current = &mut node.next;
                }
                None => break,
            }
        }
        return result;
    }
}
// @lc code=end
