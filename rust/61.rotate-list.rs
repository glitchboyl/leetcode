/*
 * @lc app=leetcode id=61 lang=rust
 *
 * [61] Rotate List
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut count = 0;
        {
            let mut p = &head;
            while let Some(node) = p {
                p = &node.next;
                count += 1;
            }
        }
        if count == 0 {
            return head;
        }
        let k = k % count;
        if k == 0 {
            return head;
        }
        let mut head = head;
        let mut p = head.as_mut();
        for _ in 0..count - k - 1 {
            p = p.unwrap().next.as_mut();
        }
        let mut dummy = p.unwrap().next.take();
        p = dummy.as_mut();
        while let Some(node) = p {
            if node.next.is_none() {
                node.next = head;
                break;
            }
            p = node.next.as_mut();
        }
        dummy
    }
}
// @lc code=end
