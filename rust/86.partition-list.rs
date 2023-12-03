/*
 * @lc app=leetcode id=86 lang=rust
 *
 * [86] Partition List
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let (mut b, mut d) = (ListNode::new(-1), ListNode::new(-1));
        let (mut p, mut q) = (&mut b, &mut d);
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            } else {
                q.next = Some(node);
                q = q.next.as_mut().unwrap();
            }
        }
        p.next = d.next.take();
        b.next
    }
}
// @lc code=end
