/*
 * @lc app=leetcode id=21 lang=javascript
 *
 * [21] Merge Two Sorted Lists
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
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
var mergeTwoLists = function (list1, list2) {
  // Solution 1: two pointers
  const dummy = new ListNode();
  let current = dummy;
  while (list1 && list2) {
    if (list1.val < list2.val) {
      current.next = list1;
      list1 = list1.next;
    } else {
      current.next = list2;
      list2 = list2.next;
    }
    current = current.next;
  }
  current.next = list1 || list2;
  return dummy.next;

  // Solution 2: recursion
  // if (!list1) return list2;
  // else if (!list2) return list1;
  // else if (list1 && list2) {
  //   if (list1.val < list2.val) {
  //     list1.next = mergeTwoLists(list1.next, list2);
  //     return list1;
  //   } else {
  //     list2.next = mergeTwoLists(list1, list2.next);
  //     return list2;
  //   }
  // } else return null;
};
// @lc code=end
