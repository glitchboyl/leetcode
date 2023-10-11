/*
 * @lc app=leetcode id=206 lang=javascript
 *
 * [206] Reverse Linked List
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
var reverseList = function (head) {
  // Solution 1:
  // let revHead = null;
  // while (head) {
  //   const temp = head;
  //   head = head.next;
  //   temp.next = revHead;
  //   revHead = temp;
  // }
  // return revHead;

  // Solution 2: recursion
  let revHead = head;
  if (head && head.next) {
    revHead = reverseList(head.next);
    head.next.next = head;
    head.next = null;
  }
  return revHead;
};
// @lc code=end
