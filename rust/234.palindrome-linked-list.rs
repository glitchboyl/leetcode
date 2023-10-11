/*
 * @lc app=leetcode id=234 lang=rust
 *
 * [234] Palindrome Linked List
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut nums = Vec::new();
        let (mut slow, mut fast) = (&head, &head);
        loop {
            match (slow.as_ref(), fast.as_ref()) {
                (Some(s_node), Some(f_node)) => {
                    if let Some(f_next_node) = f_node.next.as_ref() {
                        nums.push(s_node.val);
                        slow = &s_node.next;
                        fast = &f_next_node.next;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        if fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
        }
        while nums.len() > 0 {
            match (nums.pop(), slow.as_ref()) {
                (Some(num), Some(s_node)) => {
                    if num != s_node.val {
                        return false;
                    }
                    slow = &s_node.next;
                }
                _ => break,
            }
        }
        return true;
    }
}
// @lc code=end
