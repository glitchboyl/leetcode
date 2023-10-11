/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
        let mut result = head.clone();
        let mut current = result.as_mut();
        while let Some(node) = current {
            let mut next = node.next.take();
            while let Some(n) = next.as_mut() {
                if n.val != node.val {
                    node.next = next;
                    break;
                }
                next = n.next.take();
            }
            current = node.next.as_mut();
        }
        return result;
    }
}
// @lc code=end
