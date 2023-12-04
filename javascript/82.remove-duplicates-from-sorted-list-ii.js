/*
 * @lc app=leetcode id=82 lang=javascript
 *
 * [82] Remove Duplicates from Sorted List II
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
  const dummy = new ListNode(-1, head);
  let p = dummy;
  while (p.next) {
    let node = p.next;
    if (node.next && node.val === node.next.val) {
      while (node.next && node.val === node.next.val) {
        node = node.next;
      }
      p.next = node.next;
    } else {
      p = node;
    }
  }
  return dummy.next;
};
// @lc code=end
