/*
 * @lc app=leetcode id=86 lang=javascript
 *
 * [86] Partition List
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
 * @param {number} x
 * @return {ListNode}
 */
var partition = function (head, x) {
  const b = new ListNode(-1),
    d = new ListNode(-1);
  let p = b,
    q = d;
  while (head) {
    const node = head;
    head = head.next;
    node.next = null;
    if (node.val < x) {
      p.next = node;
      p = p.next;
    } else {
      q.next = node;
      q = q.next;
    }
  }
  p.next = d.next;
  return b.next;
};
// @lc code=end
