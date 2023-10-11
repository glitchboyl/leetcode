/*
 * @lc app=leetcode id=234 lang=javascript
 *
 * [234] Palindrome Linked List
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
 * @return {boolean}
 */
var isPalindrome = function (head) {
  let slow = head,
    fast = head;

  // Solution 1: O(1) space
  while (fast && fast.next) {
    slow = slow.next;
    fast = fast.next.next;
  }
  let tail = null;
  while (slow) {
    const temp = slow;
    slow = slow.next;
    temp.next = tail;
    tail = temp;
  }
  while (tail) {
    if (head.val !== tail.val) return false;
    head = head.next;
    tail = tail.next;
  }

  // Solution 2: array O(n) space
  // const nums = [];
  // while (fast && fast.next) {
  //   nums.push(slow.val);
  //   slow = slow.next;
  //   fast = fast.next.next;
  // }
  // if (fast !== null) slow = slow.next;
  // while (slow) {
  //   if (slow.val !== nums.pop()) return false;
  //   slow = slow.next;
  // }

  return true;
};
// @lc code=end
