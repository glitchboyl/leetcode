/*
 * @lc app=leetcode id=110 lang=javascript
 *
 * [110] Balanced Binary Tree
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isBalanced = function (root) {
  if (!root) return true;

  let maxBalance = 0;
  function getBalance(node) {
    if (node && maxBalance < 2) {
      const leftBalance = getBalance(node.left);
      const rightBalance = getBalance(node.right);
      maxBalance = Math.max(maxBalance, Math.abs(leftBalance - rightBalance));
      return Math.max(leftBalance, rightBalance) + 1;
    }
    return 0;
  }
  getBalance(root);

  return maxBalance < 2;
};
// @lc code=end
