/*
 * @lc app=leetcode id=203 lang=javascript
 *
 * [203] Remove Linked List Elements
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
 * @param {number} val
 * @return {ListNode}
 */
var removeElements = function (head, val) {
  // Solution 1:
  // let newHead = new ListNode();
  // let p = head,
  //   q = newHead;
  // while (p) {
  //   if (p.val !== val) {
  //     q.next = p;
  //     q = q.next;
  //   }
  //   p = p.next;
  // }
  // q.next = p;
  // return newHead.next;

  // Solution 2:
  let p = head,
    prev;
  while (p) {
    if (head.val === val) {
      head = head.next;
      p = head;
    } else {
      if (p.val === val) {
        prev.next = p.next;
      } else {
        prev = p;
      }
      p = p.next;
    }
  }
  return head;
};
// @lc code=end
