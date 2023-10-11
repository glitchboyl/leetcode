/*
 * @lc app=leetcode id=83 lang=javascript
 *
 * [83] Remove Duplicates from Sorted List
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function (head) {
  if (head && head.next) {
    // Solution 1: one pointer
    // let p = head;
    // while (p && p.next) {
    //   if (p.val === p.next.val) p.next = p.next.next;
    //   else p = p.next;
    // }

    // Solution 2: slow and fast pointers
    let slow = head,
      fast = head;
    while (fast && fast.next) {
      fast = fast.next;
      if (slow.val !== fast.val) {
        slow.next = fast;
        slow = fast;
      }
    }
    slow.next = null;
  }

  return head;
};
// @lc code=end
