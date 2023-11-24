/*
 * @lc app=leetcode id=563 lang=javascript
 *
 * [563] Binary Tree Tilt
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
var findTilt = function (root) {
  let tilt = 0;
  function dfs(node) {
    if (node) {
      const l = dfs(node.left),
        r = dfs(node.right);
      tilt += Math.abs(l - r);
      return l + r + node.val;
    }
    return 0;
  }
  dfs(root);
  return tilt;
};
// @lc code=end
