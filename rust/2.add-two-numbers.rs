/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Solution 1: recursion
        // return match (l1, l2) {
        //     (Some(node), None) | (None, Some(node)) => Some(node),
        //     (Some(node1), Some(node2)) => {
        //         let mut sum = node1.val + node2.val;
        //         return Some(Box::new(match sum >= 10 {
        //             true => ListNode {
        //                 val: sum - 10,
        //                 next: Self::add_two_numbers(
        //                     Self::add_two_numbers(Some(Box::new(ListNode::new(1))), node1.next),
        //                     node2.next,
        //                 ),
        //             },
        //             false => ListNode {
        //                 val: sum,
        //                 next: Self::add_two_numbers(node1.next, node2.next),
        //             },
        //         }));
        //     }
        //     _ => None,
        // };

        // Solution 2:
        let (mut l1, mut l2) = (l1.clone(), l2.clone());
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut p = &mut dummy;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let (mut v1, mut v2) = (0, 0);
            if let Some(node) = l1 {
                v1 = node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                v2 = node.val;
                l2 = node.next;
            }
            let val = v1 + v2 + carry;
            carry = val / 10;
            p.next = Some(Box::new(ListNode::new(val % 10)));
            p = p.next.as_mut().unwrap();
        }
        return dummy.next;
    }
}
// @lc code=end
