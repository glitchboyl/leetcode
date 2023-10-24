/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut p = dummy.as_mut();
        while let Some(mut node) = p.next.take() {
            if let Some(mut next_node) = node.next.take() {
                node.next = next_node.next;
                next_node.next = Some(node);
                p.next = Some(next_node);
                p = p.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                p.next = Some(node);
                break;
            }
        }
        dummy.next
    }
}
// @lc code=end
