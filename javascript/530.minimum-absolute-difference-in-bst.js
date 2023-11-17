/*
 * @lc app=leetcode id=530 lang=javascript
 *
 * [530] Minimum Absolute Difference in BST
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
 * @return {number}
 */
var getMinimumDifference = function (root) {
  let minDiff = 2 ** 31;
  let prev = null;
  function inorderTraversal(node) {
    if (node) {
      inorderTraversal(node.left);
      if (typeof prev === "number")
        minDiff = Math.min(minDiff, node.val - prev);
      prev = node.val;
      inorderTraversal(node.right);
    }
  }
  inorderTraversal(root);
  return minDiff;
};
// @lc code=end
