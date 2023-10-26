/*
 * @lc app=leetcode id=24 lang=javascript
 *
 * [24] Swap Nodes in Pairs
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
var swapPairs = function (head) {
  if (!head || !head.next) return head;
  const dummy = new ListNode(-1, head);
  let p = dummy;
  while (p.next) {
    const q = p.next;
    if (q.next) {
      p.next = q.next;
      q.next = p.next.next;
      p.next.next = q;
    }
    p = q;
  }
  return dummy.next;
};
// @lc code=end
