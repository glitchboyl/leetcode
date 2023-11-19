/*
 * @lc app=leetcode id=543 lang=javascript
 *
 * [543] Diameter of Binary Tree
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
var diameterOfBinaryTree = function (root) {
  let d = 0;
  function dfs(node) {
    if (node) {
      let l = dfs(node.left);
      let r = dfs(node.right);
      d = Math.max(d, l + r);
      return 1 + Math.max(l, r);
    }
    return 0;
  }
  dfs(root);
  return d;
};
// @lc code=end
