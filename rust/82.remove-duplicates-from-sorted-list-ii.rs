/*
 * @lc app=leetcode id=82 lang=rust
 *
 * [82] Remove Duplicates from Sorted List II
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut p = &mut dummy;
        while let Some(mut node) = p.next.as_mut() {
            if node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                    node = node.next.as_mut().unwrap();
                }
                p.next = node.next.take();
            } else {
                p = p.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
// @lc code=end
