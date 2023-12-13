/*
 * @lc app=leetcode id=92 lang=rust
 *
 * [92] Reverse Linked List II
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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut b = &mut dummy;
        for _ in 1..left {
            if let Some(n) = b {
                b = &mut n.next;
            }
        }
        let mut p = b.as_mut().unwrap().next.take();
        let mut q = p.as_mut().unwrap().next.take();
        for _ in left..right {
            if let Some(mut n) = q {
                q = n.next;
                n.next = p;
                p = Some(n);
            }
        }
        let mut d = &mut p;
        for _ in left..right {
            if let Some(n) = d {
                d = &mut n.next;
            }
        }
        d.as_mut().unwrap().next = q;
        b.as_mut().unwrap().next = p;
        dummy.unwrap().next
    }
}
// @lc code=end
