/*
 * @lc app=leetcode id=61 lang=javascript
 *
 * [61] Rotate List
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
 * @param {number} k
 * @return {ListNode}
 */
var rotateRight = function (head, k) {
  if (k === 0 || head === null) return head;
  let count = 0,
    p = head;
  while (p) {
    p = p.next;
    count++;
  }
  k %= count;
  if (k === 0) return head;
  p = head;
  for (let _ = 0; _ < count - k - 1; _++) {
    p = p.next;
  }
  let q = p.next;
  while (q.next) {
    q = q.next;
  }
  q.next = head;
  head = p.next;
  p.next = null;
  return head;
};
// @lc code=end
