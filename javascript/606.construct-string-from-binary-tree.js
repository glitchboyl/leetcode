/*
 * @lc app=leetcode id=606 lang=javascript
 *
 * [606] Construct String from Binary Tree
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
 * @return {string}
 */
var tree2str = function (root) {
  let str = "";
  if (root) {
    str += root.val;
    if (root.left || root.right) {
      str += `(${tree2str(root.left)})`;
      if (root.right) {
        str += `(${tree2str(root.right)})`;
      }
    }
  }
  return str;
};
// @lc code=end
