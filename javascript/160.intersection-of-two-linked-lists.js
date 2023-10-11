/*
 * @lc app=leetcode id=160 lang=javascript
 *
 * [160] Intersection of Two Linked Lists
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} headA
 * @param {ListNode} headB
 * @return {ListNode}
 */
var getIntersectionNode = function (headA, headB) {
  let p = headA,
    q = headB;
  let pSwitched = false, qSwitched = false;
  while (p && q) {
    if (p === q) return p;

    p = p.next;
    if (!p && !pSwitched) {
      p = headB;
      pSwitched = true;
    }
    q = q.next;
    if (!q && !qSwitched) {
      q = headA;
      qSwitched = true;
    }
  }
  return null;
};
// @lc code=end
