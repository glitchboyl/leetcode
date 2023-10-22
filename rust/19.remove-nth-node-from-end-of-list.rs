/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut p = dummy.clone();
        let mut q = dummy.as_mut();
        for _ in 0..n {
            if let Some(pn) = p.next {
                p = pn;
            }
        }
        while let Some(pn) = p.next {
            p = pn;
            q = q.next.as_mut().unwrap();
        }
        q.next = q.next.as_mut().unwrap().next.take();
        dummy.next
    }
}
// @lc code=end
